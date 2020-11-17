const {log} = console;


async function compareGas(hamester, ckbCrypto) {
  let res;

  // calc blake2b 208 gas
  res = await hamester.callStatic.test208();
  log(`Hamster blake2b gasUsed: ${res}`)

  // calc CKBCrypto.digest gas:
  res = await ckbCrypto.callStatic.ckbBlake2bGas();
  log(`CKBCrypto blake2b gasUsed: ${res}`)
}

async function main() {
  // Buidler always runs the compile task when running scripts through it.
  // If this runs in a standalone fashion you may want to call compile manually
  // to make sure everything is compiled
  // await bre.run('compile');

  // 1. deploy hamsterContract
  const factory = await ethers.getContractFactory(
    "contracts/HasmterBlake2b.sol:HasmterBlake2b"
  );
  const hamsterContract = await factory.deploy();
  await hamsterContract.deployed();

  // calc blake2b 64
  let res = await hamsterContract.callStatic.digest64(
    "0x39e33c8ad2e7e4eb71610d2bcdfbb0cb0fde2f96418256914ad2f5be1d6e9331385dfb0153a0e3aec760120c4e333a4a6bec91eeaca359ef714709588d23ca16"
  );
  assert(
    res ===
      "0x93a9faceb827e8a431217f0e5fc6068c14cc62ac4cf73752ed4e9135adc364c8",
    `${res} !== 0x93a9faceb827e8a431217f0e5fc6068c14cc62ac4cf73752ed4e9135adc364c8`
  );


  // calc blake2b 208
  res = await hamsterContract.callStatic.digest208(
    "0x0000000085b9111a7a19447c6e010000d00700000000000001000001010807005ad29fdd662e6461851eb3229154110dc3ed5c070e27a8a15095671d36c50f21d32b42b52594b4596cb2d3b5ed0ca08ac0f7671fee353e8ef8cac93f71437f6900000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f0686a287025a22e6b9138082387230012c081cb3e3600000007a58c4846ff063253ecfa0000000000000001bce9a710"
  );
  assert(
    res ===
      "0x0e497e10691ae8f4def85cbe88e3a6537a73d5793a4bed0800e5c5bb5799c24f",
    `${res} !== 0x0e497e10691ae8f4def85cbe88e3a6537a73d5793a4bed0800e5c5bb5799c24e`
  );

  // compare gas between hamster and ckbCrypto
  // 2. deploy ckbCrypto test contract
  const cryptoFactory = await ethers.getContractFactory(
      "contracts/test/TestBlake2b.sol:TestBlake2b"
  );
  const ckbCrypto = await cryptoFactory.deploy();
  await ckbCrypto.deployed();

  // 3. compare gas
  await compareGas(hamsterContract, ckbCrypto);

}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });

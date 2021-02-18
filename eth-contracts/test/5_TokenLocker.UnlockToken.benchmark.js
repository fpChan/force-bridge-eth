const { expect } = require('chai');
const { keccak256, defaultAbiCoder, toUtf8Bytes } = ethers.utils;
const {
  log,
  waitingForReceipt,
  generateSignatures,
  generateWallets,
  getMsgHashForAddHistoryTxRoot,
} = require('./utils');
const { addHistoryTxRootTestCases } = require('./data/testHistoryTxRoot.json');
const viewHistoryTxProof = require('./data/testHistoryTxProof.json');
const testJson = require('./data/testTokenLocker.json');
const recipientCellTypescript = testJson.recipientCellTypescript;
const lightClientTypescriptHash = testJson.lightClientTypescriptHash;
const bridgeCellLockscriptCodeHash = testJson.bridgeCellLockscriptCodeHash;
const decodeBurnTxTestCases = testJson.decodeBurnTxTestCases;
const lockETHTestCases = testJson.lockETHTestCases;
const lockTokenTestCases = testJson.lockTokenTestCases;
const unlockTokenTestCase = require('./data/testUnlockTokenParam.json');
let tokenLocker, provider, user;
const {
  txCntFromProof,
} = require('../scripts/benchmark/data/testBenchUnlockTokenParam.json');

contract('TokenLocker', () => {
  let ckbChain,
    adminAddress,
    contractAddress,
    initHeaderIndex,
    endHeaderIndex,
    factory;
  let wallets, validators;
  let multisigThreshold, chainId, DOMAIN_SEPARATOR, addHistoryTxRootTypeHash;
  let initBlockNumber,
    latestBlockNumber,
    historyTxRoot,
    txRootProofDataVec,
    input;

  before(async function () {
    // disable timeout
    this.timeout(0);
    const [signer] = await ethers.getSigners();
    adminAddress = signer.address;

    // get validators
    wallets = generateWallets(7);
    validators = wallets.map((wallet) => wallet.address);
    multisigThreshold = 5;
    chainId = await signer.getChainId();

    // deploy CKBChain
    factory = await ethers.getContractFactory(
      'contracts/CKBChain.sol:CKBChain'
    );

    ckbChain = await factory.deploy();
    await ckbChain.deployTransaction.wait(1);
    let res = await ckbChain.initialize(validators, multisigThreshold);
    await res.wait(1);

    // deploy TokenLocker
    factory = await ethers.getContractFactory(
      'contracts/test/TokenLocker-bench.sol:TokenLockerBench'
    );
    tokenLocker = await factory.deploy();
    await tokenLocker.deployTransaction.wait(1);
    res = await tokenLocker.initialize(
      ckbChain.address,
      0,
      recipientCellTypescript.codeHash,
      recipientCellTypescript.hashType,
      lightClientTypescriptHash,
      bridgeCellLockscriptCodeHash
    );
    await res.wait(1);

    contractAddress = tokenLocker.address;
    provider = tokenLocker.provider;
    user = tokenLocker.signer;
  });

  describe('ckbChain addHistoryTxRoot', async function () {
    // disable timeout
    this.timeout(0);

    it('SIGNATURE_SIZE, name, AddHistoryTxRootTypeHash, DOMAIN_SEPARATOR', async () => {
      expect(await ckbChain.SIGNATURE_SIZE()).to.eq(65);

      const name = 'Force Bridge CKBChain';
      expect(await ckbChain.NAME_712()).to.eq(name);

      addHistoryTxRootTypeHash = keccak256(
        toUtf8Bytes(
          'AddHistoryTxRoot(uint64 startBlockNumber, uint64 endBlockNumber, bytes32 historyTxRoot)'
        )
      );
      log(`addHeadersTypeHash`, addHistoryTxRootTypeHash);
      expect(await ckbChain.ADD_HISTORY_TX_ROOT_TYPEHASH()).to.eq(
        addHistoryTxRootTypeHash
      );

      DOMAIN_SEPARATOR = keccak256(
        defaultAbiCoder.encode(
          ['bytes32', 'bytes32', 'bytes32', 'uint256', 'address'],
          [
            keccak256(
              toUtf8Bytes(
                'EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)'
              )
            ),
            keccak256(toUtf8Bytes(name)),
            keccak256(toUtf8Bytes('1')),
            chainId,
            ckbChain.address,
          ]
        )
      );
      expect(await ckbChain.DOMAIN_SEPARATOR()).to.eq(DOMAIN_SEPARATOR);
    });

    it('use v1 contract, addHistoryTxRoot correct case', async () => {
      let actualTipNumber;
      for (const testCase of addHistoryTxRootTestCases) {
        input = testCase.input;
        initBlockNumber = testCase.initBlockNumber;
        latestBlockNumber = testCase.latestBlockNumber;
        historyTxRoot = testCase.historyTxRoot;
        txRootProofDataVec = testCase.txRootProofDataVec;

        // 1. calc msgHash
        const msgHash = getMsgHashForAddHistoryTxRoot(
          DOMAIN_SEPARATOR,
          addHistoryTxRootTypeHash,
          initBlockNumber,
          latestBlockNumber,
          historyTxRoot
        );

        // 2. generate signatures
        let signatures = generateSignatures(
          msgHash,
          wallets.slice(0, multisigThreshold)
        );

        // 3. addHeaders with gc
        const tx = await ckbChain.addHistoryTxRoot(
          initBlockNumber,
          latestBlockNumber,
          historyTxRoot,
          signatures
        );
        const receipt = await tx.wait(1);
        expect(receipt.status).to.eq(1);
        log(`gas cost: ${receipt.gasUsed}`);

        // check if addHeaders success
        actualTipNumber = await ckbChain.callStatic.latestBlockNumber();
        log(`current tipBlockNumber: ${actualTipNumber}\r\n`);
      }
    });
  });

  describe('tokenLocker test case', async function () {
    // disable timeout
    this.timeout(0);

    it('unlock benchmark', async () => {
      let receipt, size, res;
      // 4. benchmark
      const rawTx =
        '0x5e0100001c0000002000000024000000280000002c000000b6000000000000000000000000000000000000008a000000080000008200000010000000180000004d00000000000000000000003500000010000000300000003100000000000000000000000000000000000000000000000000000000000000000000000000000000350000001000000030000000310000008b79673c89404c1d98874fef0441bf25d99ed5c4e96d1180d72b203966e6c9640000000000a8000000080000009c0000001a674fdde714fd979de3edf0f56aa9716b898ec80000000000000000000000000000000000000000899ee5d8fb3875f6d9288f700917d8a7c5d51e891b79673c89404c1d98874fef0441bf25d99ed5c4e96d1180d72b203966e6c9641b79673c89404c1d98874fef0441bf25d99ed5c4e96d1180d72b203966e6c964b822000000000000000000000000000058000000000000000000000000000000';
      log('begin decodeBurnResult', await tokenLocker.decodeBurnResult(rawTx));
      for (const benchCase of txCntFromProof) {
        res = await tokenLocker.unlockToken(benchCase.input, rawTx);
        receipt = await res.wait(1);
        size = benchCase.output;
        log(
          `unlockToken with ${size} txs, gasUsed: ${
            receipt.gasUsed
          }, gas cost per tx: ${receipt.gasUsed / size}`
        );
      }
    });
  });
});

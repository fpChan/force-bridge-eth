const { expect } = require('chai');

contract('MockSpv', () => {
  let mockSpv;

  before(async () => {
    const factory = await ethers.getContractFactory('MockCKBSpv');
    mockSpv = await factory.deploy();
    await mockSpv.deployed();
  });

  describe('ProveTxExist', () => {
    it("Should return the new greeting once it's changed", async () => {
      expect(await mockSpv.proveTxExist([0], 0)).to.equal(true);
    });
  });
});

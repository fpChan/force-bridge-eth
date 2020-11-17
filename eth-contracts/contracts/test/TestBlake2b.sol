pragma solidity ^0.5.10;
import {CKBCrypto} from "../libraries/CKBCrypto.sol";

contract TestBlake2b {
    function ckbBlake2b(bytes32 left, bytes32 right) public view returns(bytes32) {
        return CKBCrypto.digest(abi.encodePacked(left, right, new bytes(64)), 64);
    }

    function ckbBlake2bGas() public view returns(uint){
        bytes memory b = hex"0000000085b9111a7a19447c6e010000d00700000000000001000001010807005ad29fdd662e6461851eb3229154110dc3ed5c070e27a8a15095671d36c50f21d32b42b52594b4596cb2d3b5ed0ca08ac0f7671fee353e8ef8cac93f71437f6900000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f0686a287025a22e6b9138082387230012c081cb3e3600000007a58c4846ff063253ecfa0000000000000001bce9a710";
        uint before = gasleft();
        bytes32 ret = CKBCrypto.digest(b, 208);
        return before - gasleft();
    }
}

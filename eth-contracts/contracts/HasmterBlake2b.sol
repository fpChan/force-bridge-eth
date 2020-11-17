// SPDX-License-Identifier: MIT
pragma solidity ^0.5.10;

contract HasmterBlake2b {
    constructor() public{

    }
    //955c1b5a510509161a657fe18d67794291ab2a6f28587478593353970a00d47f
    //event Test(bytes fdata);

    event Result(bytes32 ret);


    function digest208(bytes calldata a) external returns (bytes32) {
        assembly{

            // reject not ckbinput
            // 4 + 32 + 32 + 208
            if lt(calldatasize(), 0x0114){
                revert(0x80,0x00)
            }

            if eq(eq(calldataload(0x04), 0x20),0){
                revert(0x80,0x00)
            }

            if eq(eq(calldataload(0x24), 0x00000000000000000000000000000000000000000000000000000000000000d0),0){
                revert(0x80,0x00)
            }

        // the init vector is 0x
        //  08c9bc3f 67e6096a 3ba7ca84 85ae67bb
        //  2bf894fe 72f36e3c f1361d5f 3af54fa5
        //  d182e6ad 7f520e51 1f6c3e2b 8c68059b
        //  6bbd41fb abd9831f 79217e13 19cde05b
        //
        // the param is
        //         let param = blake2b_param {
        //            digest_length: out_len as u8,   u8 0x20
        //            key_length: 0, u8 0x00
        //            fanout: 1, u8 0x01
        //            depth: 1,  u8 0x01
        //            leaf_length: 0, u32
        //            node_offset: 0,u32
        //            xof_length: 0, u32
        //            node_depth: 0, u8
        //            inner_length: 0, u8
        //            reserved: [0u8; 14usize], u8[14]
        //            salt: [0u8; blake2b_constant_BLAKE2B_SALTBYTES as usize], u8[16]
        //            personal: [0u8; blake2b_constant_BLAKE2B_PERSONALBYTES as usize], u8[16]
        //        };
        // pub const blake2b_constant_BLAKE2B_SALTBYTES: blake2b_constant = 16;
        // pub const blake2b_constant_BLAKE2B_PERSONALBYTES: blake2b_constant = 16;
        // the PERSONALBYTES is b"ckb-default-hash";
        // PERSONALBYTES = 636b622d 64656661 756c742d 68617368
        //
        // the param is 0x
        // 20000101 00000000 00000000 00000000 [digest_length key_length fanout depth] leaf_length node_offset xof_length
        // 00000000 00000000 00000000 00000000 node_depth inner_length reserved
        // 00000000 00000000 00000000 00000000 salt
        // 636b622d 64656661 756c742d 68617368 personal
        //
        // iv ^ param is 64 bytes, which is the init h
        // 28c9bdf2 67e6096a 3ba7ca84 85ae67bb
        // 2bf894fe 72f36e3c f1361d5f 3af54fa5
        // d182e6ad 7f520e51 1f6c3e2b 8c68059b
        // 08d623d6 cfbce57e 0c4d0a3e 71ac9333
        //
        // param for blake2b F():
        // rounds - the number of rounds - 32-bit unsigned big-endian word
        // h - the state vector - 8 unsigned 64-bit little-endian words
        // m - the message block vector - 16 unsigned 64-bit little-endian words
        // t_0, t_1 - offset counters - 2 unsigned 64-bit little-endian words
        // f - the final block indicator flag - 8-bit word
        //
        // the rounds === 12 for blake2b, 10 for blake2s
        // h is state vector, the first/init is iv^param as above
        // m, t_0, t_1 and f is initialized as 0
        // the first call param is
        // 0000000c28c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b08d623d6cfbce57e0c4d0a3e71ac933300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
        //
        // we don't have private/password 'key', so we can skip 'first' update loop
        //
        // 208 = 128 + 80 + 28 padding
        // it need 7slot * 32(256bits) = 224 bytes to cover m

        // due to call/staticcall/delegagecall, we have to use memory
        // 0x80 + 213 = 341 0x0155
        // 0x80 + 0xe0 = 0x0160, we align memory to 0x20 bytes due to evm is 256-bit machine
        // take over memory management
            mstore(0x40, 0x0160)

        //        round   h
        //   0x80 0000000c28c9bdf267e6096a3ba7ca84
        //   0x90 85ae67bb2bf894fe72f36e3cf1361d5f

        //   0xA0 3af54fa5d182e6ad7f520e511f6c3e2b
        //   0xB0 8c68059b08d623d6cfbce57e0c4d0a3e

        //                m
        //   0xC0 71ac9333000000000000000000000000
        //   0xD0 00000000000000000000000000000000

        //   0xE0 00000000000000000000000000000000
        //   0xF0 00000000000000000000000000000000

        // 0x0100 00000000000000000000000000000000
        // 0x0110 00000000000000000000000000000000

        // 0x0120 00000000000000000000000000000000
        // 0x0130 00000000000000000000000000000000
        //
        //                t0              t1
        // 0x0140 00000000000000000000000000000000
        // 0x0150 0000000000PPPPPPPPPPPPPPPPPPPPPP  P for placeholder
        //                fi
        // populate init param
            mstore(0x80, 0x0000000c28c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f)
            mstore(0xA0, 0x3af54fa5d182e6ad7f520e511f6c3e2b8c68059b08d623d6cfbce57e0c4d0a3e)
            mstore(0xC0, 0x71ac933300000000000000000000000000000000000000000000000000000000)
            mstore(0xE0, 0x0000000000000000000000000000000000000000000000000000000000000000)
            mstore(0x0100, 0x0000000000000000000000000000000000000000000000000000000000000000)
            mstore(0x0120, 0x0000000000000000000000000000000000000000000000000000000000000000)
            mstore(0x0140, 0x0000000000000000000000000000000000000000000000000000000000000000)


        // copy 128 0x80 bytes to m, eliminate selector and leading length of bytes
            calldatacopy(0xC4, 0x44, 0x80)

        // set t0,t1 to 128 0x80
        // watch that the data is Little.Endian
        // t0 = 0x 80 00 00 00 00 00 00 00
        // t1 = 0x 00 00 00 00 00 00 00 00
            mstore8(0x0144,0x80)

        // not final block, leave f to 0x00

        // call F()

        // pass memory to blake2b, get the result h at 0x80+0x04, over-writing
            log1(0x80,0xD5,0x955c1b5a510509161a657fe18d67794291ab2a6f28587478593353970a00d47f)
            if iszero(staticcall(not(0), 0x09, 0x80, 0xd5, 0x84, 0x40)) {
                revert(0x80, 0x00)
            }
            log1(0x80,0xD5,0x955c1b5a510509161a657fe18d67794291ab2a6f28587478593353970a00d47f)

        // the remaining 208-128=80 0x50 bytes input data

        // copy 208-128=80 0x50 bytes to m, need padding zero
            calldatacopy(0xC4, 0xC4, 0x50)
            mstore(0x0114,0x0000000000000000000000000000000000000000000000000000000000000000)
            mstore(0x0120,0x0000000000000000000000000000000000000000000000000000000000000000)
            mstore(0x0140,0x0000000000000000000000000000000000000000000000000000000000000000)

        // set t0,t1 to 208 0xD0
        // watch that the data is Little.Endian
        // t0 = 0x 80 00 00 00 00 00 00 00
        // t1 = 0x 00 00 00 00 00 00 00 00
            mstore8(0x0144,0xD0)

        // final block, set f to 0x01
            mstore8(0x0154,0x01)

        // call F()
        // pass memory to blake2b, get the result h at 0x80+0x04, over-writing
            log1(0x80,0xD5,0x955c1b5a510509161a657fe18d67794291ab2a6f28587478593353970a00d47f)
            if iszero(staticcall(not(0), 0x09, 0x80, 0xd5, 0x84, 0x40)) {
                revert(0x80, 0x00)
            }
            log1(0x80,0xD5,0x955c1b5a510509161a657fe18d67794291ab2a6f28587478593353970a00d47f)

            return(0x84,0x20)
        }
    }

    function digest64(bytes calldata a) external returns (bytes32){
        assembly{
            // reject not ckbinput
            // 4 + 32 + 32 + 64
            if lt(calldatasize(), 0x84){
                revert(0x80,0x00)
            }

            if eq(eq(calldataload(0x04), 0x20),0){
                revert(0x80,0x00)
            }

            if eq(eq(calldataload(0x24), 0x0000000000000000000000000000000000000000000000000000000000000040),0){
                revert(0x80,0x00)
            }

            mstore(0x80, 0x0000000c28c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f)
            mstore(0xA0, 0x3af54fa5d182e6ad7f520e511f6c3e2b8c68059b08d623d6cfbce57e0c4d0a3e)
            mstore(0xC0, 0x71ac933300000000000000000000000000000000000000000000000000000000)
            mstore(0xE0, 0x0000000000000000000000000000000000000000000000000000000000000000)
            mstore(0x0100, 0x0000000000000000000000000000000000000000000000000000000000000000)
            mstore(0x0120, 0x0000000000000000000000000000000000000000000000000000000000000000)
            mstore(0x0140, 0x0000000000000000000000000000000000000000000000000000000000000000)


        // copy 64 0x40 bytes to m, eliminate selector and leading length of bytes
        // the remaining m is zero as initialized, as same as padding
            calldatacopy(0xC4, 0x44, 0x40)

        // set t0,t1 to 64 0x40
            mstore8(0x0144,0x40)

        // final block, set f to 0x01
            mstore8(0x0154,0x01)
        // call F()

        // pass memory to blake2b, get the result h at 0x80+0x04, over-writing
            log1(0x80,0xD5,0x955c1b5a510509161a657fe18d67794291ab2a6f28587478593353970a00d47f)
            if iszero(staticcall(not(0), 0x09, 0x80, 0xd5, 0x84, 0x40)) {
                revert(0x80, 0x00)
            }
            log1(0x80,0xD5,0x955c1b5a510509161a657fe18d67794291ab2a6f28587478593353970a00d47f)

            return(0x84,0x20)
        }
    }

    function test208() external returns (uint) {
        bytes memory b = hex"0000000085b9111a7a19447c6e010000d00700000000000001000001010807005ad29fdd662e6461851eb3229154110dc3ed5c070e27a8a15095671d36c50f21d32b42b52594b4596cb2d3b5ed0ca08ac0f7671fee353e8ef8cac93f71437f6900000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f0686a287025a22e6b9138082387230012c081cb3e3600000007a58c4846ff063253ecfa0000000000000001bce9a710";
        uint before = gasleft();
        bytes32 ret = this.digest208(b);
        return before - gasleft();
        emit Result(ret);
    }

    function test64() external{
        bytes memory b = hex"0000000085b9111a7a19447c6e010000d00700000000000001000001010807005ad29fdd662e6461851eb3229154110dc3ed5c070e27a8a15095671d36c50f21";
        bytes32 ret = this.digest64(b);
        emit Result(ret);
    }
}

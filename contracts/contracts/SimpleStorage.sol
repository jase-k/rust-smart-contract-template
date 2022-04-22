// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0; 

contract SimpleStorage {
    uint private stored_number;

    function get_stored_number() public view returns(uint) {
        return stored_number;
    }

    function set_stored_number(uint new_number) public {
        stored_number = new_number; 
    }
}
import { assert, assertEquals } from "https://deno.land/std@0.167.0/testing/asserts.ts";
import  BT from "../data_structures/binary_tree.ts";

Deno.test("BinaryTree::Insert", function() {
    let s = new BT();

    s.insert(1);
    s.insert(6);
    s.insert(23);
    s.insert(12);

    console.log(s);
});

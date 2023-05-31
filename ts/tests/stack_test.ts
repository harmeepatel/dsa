import { assert, assertEquals } from "https://deno.land/std@0.167.0/testing/asserts.ts";
import ArrayStack from "../data_structures/array_stack.ts";
import LLStack from "../data_structures/linked_list_stack.ts";

Deno.test("ArrayStack::PushingAndPoping", function() {
    let size = 4;
    let out = new Array<number>();
    let s = new ArrayStack<number>(size);

    s.push(1);
    s.push(6);
    s.push(23);
    s.push(12);
    assertEquals([1, 6, 23, 12], s.values);
    let v = s.pop();
    if (v) {
        out.push(v);
    }
    assertEquals([1, 6, 23, undefined], s.values);
    v = s.pop();
    if (v) {
        out.push(v);
    }
    assertEquals([1, 6, undefined, undefined], s.values);
    s.push(346);
    s.push(534);
    assertEquals([1, 6, 346, 534], s.values);
    s.push(5); // capacity reached
    v = s.pop();
    while (v) {
        out.push(v);
        v = s.pop(); // empty stack
    }
    assertEquals(new Array<number|undefined>(size).fill(undefined), s.values);
});

Deno.test("LLStack::PushingAndPoping", function() {
    let out = new Array<number>();
    let s = new LLStack<number>();

    s.push(1);
    s.push(6);
    s.push(23);
    s.push(12);
    assertEquals([1, 6, 23, 12], s.toArray());
    let v = s.pop();
    if (v) {
        out.push(v);
    }
    assertEquals([1, 6, 23], s.toArray());
    v = s.pop();
    if (v) {
        out.push(v);
    }
    assertEquals([1, 6], s.toArray());
    s.push(346);
    s.push(534);
    assertEquals([1, 6, 346, 534], s.toArray());
    s.push(5); // capacity reached
    v = s.pop();
    while (v) {
        out.push(v);
        v = s.pop(); // empty stack
    }
    assertEquals([], s.toArray());
});

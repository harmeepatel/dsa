import { assert, assertEquals } from "https://deno.land/std@0.167.0/testing/asserts.ts";
import LLQueue from "../data_structures/linked_list_queue.ts";

Deno.test("LLQueue::PushingAndPoping", function() {
    let out = new Array<number>();
    let s = new LLQueue<number>();

    s.enqueue(1);
    s.enqueue(6);
    s.enqueue(23);
    s.enqueue(12);
    assertEquals([1, 6, 23, 12], s.toArray());
    let v = s.dequeue();
    if (v) {
        out.push(v);
    }
    assertEquals([6, 23, 12], s.toArray());
    v = s.dequeue();
    if (v) {
        out.push(v);
    }
    assertEquals([23, 12], s.toArray());
    s.enqueue(346);
    s.enqueue(534);
    assertEquals([23, 12, 346, 534], s.toArray());
    s.enqueue(5);
    v = s.dequeue();
    while (v) {
        out.push(v);
        v = s.dequeue(); // empty queue
    }
    assertEquals([], s.toArray());
});

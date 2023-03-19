import { assertEquals } from "https://deno.land/std@0.178.0/testing/asserts.ts";
import DLL from "./doubly_linked_list.ts";

Deno.test(function push() {
    const list = new DLL({ val: 1, next: null, prev: null });
    list.push({ val: 2, next: null, prev: null });
    list.push({ val: 3, next: null, prev: null });

    assertEquals(list.toArray(), [1, 2, 3]);
});

Deno.test(function pop() {
    const list = new DLL({ val: 1, next: null, prev: null });
    list.push({ val: 2, next: null, prev: null });
    list.push({ val: 3, next: null, prev: null });
    list.push({ val: 4, next: null, prev: null });

    list.pop();
    assertEquals(list.toArray(), [1, 2, 3]);
    list.pop();
    assertEquals(list.toArray(), [1, 2]);
});

Deno.test(function pushFront() {
    const list = new DLL({ val: 1, next: null, prev: null });
    list.pushFront({ val: 2, next: null, prev: null });
    list.pushFront({ val: 3, next: null, prev: null });

    assertEquals(list.toArray(), [3, 2, 1]);
});

Deno.test(function popFront() {
    const list = new DLL({ val: 1, next: null, prev: null });
    list.pushFront({ val: 2, next: null, prev: null });
    list.pushFront({ val: 3, next: null, prev: null });

    list.popFront();

    assertEquals(list.toArray(), [2, 1]);
});

Deno.test(function insertAt() {
    const list = new DLL({ val: 1, next: null, prev: null });
    list.push({ val: 2, next: null, prev: null });
    list.push({ val: 3, next: null, prev: null });
    list.insertAt(0, { val: 10, next: null, prev: null });
    list.insertAt(2, { val: 20, next: null, prev: null });
    list.insertAt(4, { val: 30, next: null, prev: null });
    list.insertAt(6, { val: 40, next: null, prev: null });

    assertEquals(list.toArray(), [10, 1, 20, 2, 30, 3, 40]);
});

Deno.test(function removeAt() {
    const list = new DLL({ val: 1, next: null, prev: null });
    list.push({ val: 2, next: null, prev: null });
    list.push({ val: 3, next: null, prev: null });

    list.insertAt(0, { val: 10, next: null, prev: null });
    list.insertAt(2, { val: 20, next: null, prev: null });
    list.insertAt(4, { val: 30, next: null, prev: null });
    list.insertAt(6, { val: 40, next: null, prev: null });

    list.removeAt(0);
    list.removeAt(1);
    list.removeAt(2);
    list.removeAt(3);
    assertEquals(list.toArray(), [1, 2, 3]);
});

Deno.test(function replace() {
    const list = new DLL({ val: 1, next: null, prev: null });
    list.push({ val: 2, next: null, prev: null });
    list.push({ val: 3, next: null, prev: null });

    list.insertAt(0, { val: 10, next: null, prev: null });
    list.insertAt(2, { val: 20, next: null, prev: null });
    list.insertAt(4, { val: 30, next: null, prev: null });
    list.insertAt(6, { val: 40, next: null, prev: null });

    list.replace(0, 40);
    list.replace(2, 30);
    list.replace(4, 20);
    list.replace(6, 10);
    assertEquals(list.toArray(), [40, 1, 30, 2, 20, 3, 10]);
});

Deno.test(function get() {
    const list = new DLL({ val: 1, next: null, prev: null });
    list.push({ val: 2, next: null, prev: null });
    list.push({ val: 3, next: null, prev: null });

    list.insertAt(0, { val: 10, next: null, prev: null });
    list.insertAt(2, { val: 20, next: null, prev: null });
    list.insertAt(4, { val: 30, next: null, prev: null });
    list.insertAt(6, { val: 40, next: null, prev: null });

    const a = list.get(0);
    const b = list.get(list.length - 1);
    const c = list.get(3);

    assertEquals(a, 10);
    assertEquals(b, 40);
    assertEquals(c, 2);
});

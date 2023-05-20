import { assertThrows, assertEquals } from "https://deno.land/std@0.167.0/testing/asserts.ts";
import DLL from "../doubly_linked_list.ts";

Deno.test("Push", function push() {
    const list = new DLL();
    assertEquals(list.toArray(), []);

    list.push({ val: 1, next: null, prev: null });
    list.push({ val: 2, next: null, prev: null });
    list.push({ val: 3, next: null, prev: null });

    assertEquals(list.toArray(), [1, 2, 3]);
});

Deno.test("Pop", function pop() {
    const list = new DLL();
    list.push({ val: 1, next: null, prev: null });
    list.push({ val: 2, next: null, prev: null });
    list.push({ val: 3, next: null, prev: null });
    list.push({ val: 4, next: null, prev: null });

    let a = list.pop();
    assertEquals(a, 4);
    assertEquals(list.toArray(), [1, 2, 3]);

    a = list.pop();
    assertEquals(a, 3);
    assertEquals(list.toArray(), [1, 2]);

    a = list.pop();
    assertEquals(a, 2);
    assertEquals(list.toArray(), [1]);

    a = list.pop();
    assertEquals(a, 1);
    assertEquals(list.toArray(), []);

    const g = list.get(0);
    assertEquals(g, undefined);

});

Deno.test("PushFront", function pushFront() {
    const list = new DLL();
    list.push({ val: 1, next: null, prev: null });
    list.pushFront({ val: 2, next: null, prev: null });
    list.push({ val: 3, next: null, prev: null });
    list.pushFront({ val: 4, next: null, prev: null });

    assertEquals(list.toArray(), [4, 2, 1, 3]);
});

Deno.test("PopFront", function popFront() {
    const list = new DLL();
    list.push({ val: 1, next: null, prev: null });
    list.pushFront({ val: 2, next: null, prev: null });
    list.pushFront({ val: 3, next: null, prev: null });

    list.popFront();

    assertEquals(list.toArray(), [2, 1]);
});

Deno.test("InsertAt", function insertAt() {
    const list = new DLL();
    list.push({ val: 1, next: null, prev: null });
    list.push({ val: 2, next: null, prev: null });
    list.push({ val: 3, next: null, prev: null });

    list.insertAt(0, { val: 10, next: null, prev: null });
    assertEquals(list.toArray(), [10, 1, 2, 3]);

    list.insertAt(2, { val: 20, next: null, prev: null });
    assertEquals(list.toArray(), [10, 1, 20, 2, 3]);

    list.insertAt(4, { val: 30, next: null, prev: null });
    assertEquals(list.toArray(), [10, 1, 20, 2, 30, 3]);

    list.insertAt(6, { val: 40, next: null, prev: null });
    assertEquals(list.toArray(), [10, 1, 20, 2, 30, 3, 40]);

});

Deno.test("RemoveAt", function removeAt() {
    const list = new DLL();
    list.push({ val: 1, next: null, prev: null });
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

Deno.test("Replace", function replace() {
    const list = new DLL();
    list.push({ val: 1, next: null, prev: null });
    list.push({ val: 2, next: null, prev: null });
    list.push({ val: 3, next: null, prev: null });
    list.push({ val: 4, next: null, prev: null });
    list.push({ val: 5, next: null, prev: null });
    list.push({ val: 6, next: null, prev: null });
    list.push({ val: 7, next: null, prev: null });
    list.push({ val: 8, next: null, prev: null });

    list.replace(0, 40);
    list.replace(2, 30);
    list.replace(4, 20);
    list.replace(list.toArray().length - 1, 10);
    assertEquals(list.toArray(), [40, 2, 30, 4, 20, 6, 7, 10]);
});

Deno.test("Get", function get() {
    const list = new DLL();
    list.push({ val: 1, next: null, prev: null });
    list.push({ val: 2, next: null, prev: null });
    list.push({ val: 3, next: null, prev: null });
    list.push({ val: 4, next: null, prev: null });
    list.push({ val: 5, next: null, prev: null });
    list.push({ val: 6, next: null, prev: null });
    list.push({ val: 7, next: null, prev: null });
    list.push({ val: 8, next: null, prev: null });

    assertEquals(list.get(0), 1);
    assertEquals(list.get(list.length - 1), 8);
    assertEquals(list.get(3), 4);
    assertEquals(list.get(69), undefined);
});

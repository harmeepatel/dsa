import DLL from "../data_structures/doubly_linked_list.ts";
const LARGE_NUM = 4096;

const v: number[] = [];
for (let i = 0; i <= LARGE_NUM; i++) {
    v.push(Math.floor(Math.random() * 1000));
}

// push
let list = new DLL<number>();
Deno.bench(function push() {
    for (let i of v) {
        list.push({ val: i, next: null, prev: null });
    }
});

// pop
list = new DLL<number>();
for (let i of v) {
    list.push({ val: i, next: null, prev: null });
}
Deno.bench(function pop() {
    for (let _ in v) {
        list.pop();
    }
});

// pushFront
list = new DLL<number>();
Deno.bench(function pushFront() {
    for (let i of v) {
        list.pushFront({ val: i, next: null, prev: null });
    }
});

// popFront
list = new DLL<number>();
for (let i of v) {
    list.push({ val: i, next: null, prev: null });
}
Deno.bench(function popFront() {
    for (let _ in v) {
        list.popFront();
    }
});

// TODO: Fix this!!!
// insertAt
list = new DLL<number>();
Deno.bench(function insertAt() {
    // list.push({ val: 1, next: null, prev: null });
    // list.push({ val: 2, next: null, prev: null });
    // list.push({ val: 3, next: null, prev: null });
    // list.push({ val: 4, next: null, prev: null });
    // for (let i of v) {
    //     // let idx: number = Math.floor(Math.random() * list.length)
    //     list.insertAt(0, { val: i, next: null, prev: null })
    // }
});

// removeAt
list = new DLL<number>();
for (let i of v) {
    list.push({ val: i, next: null, prev: null });
}
Deno.bench(function removeAt() {
    // for (let _ in v){
    //     let idx:number = Math.floor(Math.random() * list.length)
    //     list.removeAt(idx)
    // }
});

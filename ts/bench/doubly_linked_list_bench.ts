import DLL from "../doubly_linked_list.ts";
const LARGE_NUM = 2048;

let list = new DLL();
const v: number[] = [];

for (let i = 0; i <= LARGE_NUM; i++) {
    v.push(Math.floor(Math.random() * 1000));
}

Deno.bench(function push() {
    for (let i in v) {
        list.push({ val: parseInt(i), next: null, prev: null });
    }
});

for (let i in v) {
    list.push({ val: parseInt(i), next: null, prev: null });
}
Deno.bench(function pop() {
    for (let _ in v) {
        list.pop();
    }
});
list = new DLL();


Deno.bench(function pushFront() {
    for (let i in v) {
        list.pushFront({ val: parseInt(i), next: null, prev: null });
    }
});

for (let i in v) {
    list.push({ val: parseInt(i), next: null, prev: null });
}
Deno.bench(function popFront() {
    for (let _ in v) {
        list.popFront();
    }
});
list = new DLL();

// Deno.bench(function insertAt() {
//     for (let i in v){
//         let idx:number = Math.floor(Math.random() * list.length)
//         list.insertAt(idx, { val: parseInt(i), next: null, prev: null })
//     }
// });
//
// for (let i in v) {
//     list.push({ val: parseInt(i), next: null, prev: null });
// }
// Deno.bench(function removeAt() {
//     for (let _ in v){
//         let idx:number = Math.floor(Math.random() * list.length)
//         list.removeAt(idx)
//     }
// });
// list = new DLL({ val: 1, next: null, prev: null });

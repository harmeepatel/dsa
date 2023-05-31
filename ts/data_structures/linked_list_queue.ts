type Node<T> = {
    val: T,
    next: Node<T> | null,
    prev: Node<T> | null,
}
export default class Queue<T> {
    head: Node<T> | null;
    tail: Node<T> | null;
    length: number;

    constructor() {
        this.head = null;
        this.tail = null;
        this.length = 0;
    }

    enqueue(val: T) {
        let new_node = {
            val,
            next: null,
            prev: null
        } as Node<T>;
        if (!this.tail) {
            this.head = new_node;
            this.tail = new_node;
            ++this.length;
            return;
        }
        this.tail.next = new_node;
        new_node.prev = this.tail;
        this.tail = new_node;
        ++this.length;
    }

    dequeue(): T | undefined {
        if (!this.head) {
            console.log("Queue is Empty!");
            return undefined;
        }
        let curr = this.head;
        if (!curr.next) {
            let val = curr.val;
            this.head = null;
            this.tail = null;
            --this.length;
            return val;
        }
        this.head = curr.next;
        this.head.prev = null;
        curr.next = null;
        --this.length;
        return curr.val;
    }

    toArray(): T[] {
        let curr = this.head;
        let out = new Array<T>();
        while (curr) {
            out.push(curr.val);
            curr = curr.next;
        }
        return out;
    }
}

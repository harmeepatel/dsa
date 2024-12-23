type Node<T> = {
    val: T,
    next: Node<T> | null,
    prev: Node<T> | null,
}
export default class Stack<T> {
    head: Node<T> | null;
    length: number;

    constructor() {
        this.head = null;
        this.length = 0;
    }

    push(val: T) {
        if (!this.head) {
            this.head = {
                val,
                next: null,
                prev: null
            } as Node<T>;
            ++this.length;
            return;
        }
        let new_node = { val, next: null, prev: this.head } as Node<T>;
        this.head.next = new_node;
        this.head = new_node;
        ++this.length;
    }

    pop(): T | undefined {
        if (!this.head) {
            console.log("Stack is Empty!");
            return undefined;
        }
        let curr = this.head;
        if (!curr.prev) {
            let val = this.head.val;
            this.head = null;
            --this.length;
            return val;
        }
        this.head = curr.prev;
        this.head.next = null;
        curr.prev = null;
        --this.length;
        return curr.val;
    }

    toArray(): T[] {
        let curr = this.head;
        let out = new Array<T>();
        while (curr) {
            out.push(curr.val);
            curr = curr.prev;
        }
        return out.reverse();
    }
}

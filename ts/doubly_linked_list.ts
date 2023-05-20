type OptNode<T> = Node<T> | null;

type Node<T> = {
    val: T | undefined;
    next: OptNode<T>;
    prev: OptNode<T>;
};

export default class DLL<T> {
    head: OptNode<T>;
    tail: OptNode<T>;
    length: number;

    constructor() {
        this.head = null;
        this.tail = null;
        this.length = 0;
    }

    push(node: Node<T>) {
        ++this.length;
        if (!this.tail) {
            this.tail = node;
            this.head = node;
            return;
        }

        this.tail.next = node;
        node.prev = this.tail;
        this.tail = node;
    }

    pop(): T | undefined {
        --this.length;

        let popElem = this.tail;
        if (popElem) {
            this.tail = popElem.prev;
            popElem.prev = null;
        }
        if (this.tail) {
            this.tail.next = null;
        }
        return popElem ? popElem.val : undefined;
    }

    pushFront(node: Node<T>) {
        ++this.length;

        if (!this.head) {
            this.head = node;
            this.tail = node;
            return;
        }

        node.next = this.head;
        this.head.prev = node;
        this.head = node;
    }

    popFront(): T | undefined {
        --this.length;

        let popElem = this.head;
        if (popElem) {
            this.head = popElem.next;
            popElem.next = null;
        }
        if (this.head) {
            this.head.prev = null;
        }
        return popElem ? popElem.val : undefined;
    }

    insertAt(idx: number, node: Node<T>) {
        if (idx > this.length) {
            console.log("Invaild Index!");
            return;
        }
        if (idx === 0) {
            this.pushFront(node);
            return;
        }
        if (idx === this.length) {
            this.push(node);
            return;
        }

        ++this.length;
        let startFromHead = idx < this.length / 2;
        idx = startFromHead ? idx : this.length - 1 - idx;

        let curr = startFromHead ? this.head : this.tail;
        for (let i = 0; curr && i < idx - 1; ++i) {
            curr = startFromHead ? curr.next : curr.prev;
        }
        if (curr) {
            node.prev = curr;
            node.next = curr.next;
            if (curr.next) {
                curr.next.prev = node;
                curr.next = node;
            } else {
                curr.next = { val: curr.val, next: null, prev: curr } as Node<T>;
                this.tail = curr.next;
                curr.val = node.val;
            }
        }
    }

    removeAt(idx: number) {
        if (idx >= this.length) {
            console.log("Invalid Index!");
            return;
        }
        if (idx === 0) {
            this.popFront();
            return;
        }
        if (idx === this.length - 1) {
            this.pop();
            return;
        }

        let startFromHead = idx < this.length / 2;
        let curr = startFromHead ? this.head : this.tail;

        for (let i = 0; i < idx; ++i) {
            if (curr) {
                curr = startFromHead ? curr.next : curr.prev;
            } else {
                console.log("Something is wrong...");
            }
        }
        if (curr && curr.next && curr.prev) {
            curr.next.prev = curr.prev;
            curr.prev.next = curr.next;
            curr.next = null;
            curr.prev = null;
            --this.length;
        }
    }

    replace(idx: number, val: T) {
        if (idx >= this.length) {
            // console.log("Invaild Index!");
            return;
        }
        let startFromHead = idx < this.length / 2;
        idx = startFromHead ? idx : this.length - idx - 1;

        let curr = startFromHead ? this.head : this.tail;
        for (let i = 0; curr && i < idx; ++i) {
            curr = startFromHead ? curr.next : curr.prev;
        }
        if (curr) {
            curr.val = val;
        }
    }

    get(idx: number): T | undefined {
        if (idx >= this.length) {
            // console.log("Invaild Index!");
            return;
        }
        let startFromHead = idx < this.length / 2;
        idx = startFromHead ? idx : this.length - idx - 1;

        let curr: OptNode<T> = startFromHead ? this.head : this.tail;
        for (let i = 0; curr && i < idx; ++i) {
            curr = startFromHead ? curr.next : curr.prev;
        }
        return curr ? curr.val : undefined;
    }

    toArray() {
        let arr = new Array<T | undefined>();
        let curr: OptNode<T> = this.head;
        for (let i = 0; curr && i < this.length; ++i) {
            arr.push(curr.val);
            curr = curr.next;
        }
        return arr;
    }

    print() {
        let curr: OptNode<T> = this.head;
        console.log(`Head: ${this.head ? this.head.val : null}; Tail: ${this.tail ? this.tail.val : null}`);
        while (curr) {
            console.log(`Node: ${curr.val}; Node[prev]: ${curr.prev ? curr.prev.val : null}; Node[next]: ${curr.next ? curr.next.val : null}`);
            curr = curr.next;
        }
    }
}

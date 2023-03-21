type Item<T> = {
    val: T;
    next: Item<T> | null;
    prev: Item<T> | null;
};

export default class DLL<T> {
    head: Item<T>;
    tail: Item<T>;
    length: number;

    constructor(node: Item<T>) {
        this.head = node;
        this.tail = node;
        this.length = 1;
    }

    push(node: Item<T>) {
        this.tail.next = node;
        node.prev = this.tail;
        this.tail = node;

        this.length++;
    }

    pop() {
        if (this.tail.prev) {
            this.tail = this.tail.prev;
        }
        if (this.tail.next) {
            this.tail.next.prev = null;
        }
        this.tail.next = null;

        this.length--;
    }

    pushFront(node: Item<T>) {
        let cur = this.head;
        while (cur.prev) {
            cur = cur.prev;
        }
        cur.prev = node;
        node.next = cur;
        this.head = node;

        this.length++;
    }

    popFront() {
        if (this.head.next) {
            this.head = this.head.next;
            if (this.head.prev) {
                this.head.prev.next = null;
            }
            this.head.prev = null;
        }
        this.length--;
    }

    insertAt(idx: number, node: Item<T>) {
        if (idx > this.length) {
            console.log(
                `List length(${this.length}) exceeded. Cannot insert ${node.val} at ${idx}`,
            );
            return;
        }
        if (idx === this.length) {
            this.push(node);
            return;
        }
        if (idx === 0) {
            this.pushFront(node);
            return;
        }
        let c = 1;
        let cur = this.head;
        while (cur.next) {
            if (c === idx) {
                node.next = cur.next;
                node.prev = cur;
                if (cur.next?.prev) {
                    cur.next.prev = node;
                }
                cur.next = node;

                this.length++;
                break;
            }
            cur = cur.next;
            ++c;
        }
    }

    removeAt(idx: number) {
        if (idx > this.length) {
            console.log(
                `List length(${this.length}) exceeded. Cannot remove ${idx}`,
            );
            return;
        }
        if (idx + 1 === this.length) {
            this.pop();
            return;
        }
        if (idx === 0 && this.head.next) {
            this.popFront();
            return;
        }
        let c = 1;
        let cur = this.head.next;
        if (cur) {
            while (cur.next) {
                if (c === idx) {
                    if (cur.prev) {
                        cur.prev.next = cur.next;
                    }
                    if (cur.next) {
                        cur.next.prev = cur.prev;
                    }

                    this.length--;
                    break;
                }
                cur = cur.next;
                c++;
            }
        }
    }

    replace(idx: number, val: T) {
        if (idx > this.length) {
            console.log(
                `List length(${this.length}) exceeded. Cannot replace at ${idx}`,
            );
            return;
        }
        let c = 0;
        let cur = this.head;
        while (cur.next) {
            if (c === idx) {
                cur.val = val
                break;
            }
            cur = cur.next;
            c++;
        }
        cur.val = val;
    }

    get(idx: number) {
        let c = 0;
        let cur = this.head;
        while (cur.next) {
            if (c === idx) {
                return cur.val;
            }
            cur = cur.next;
            c++;
        }
        return cur.val;
    }

    toArray() {
        const arr: T[] = [];
        let cur = this.head;
        while (cur.next) {
            arr.push(cur.val);
            cur = cur.next;
        }
        arr.push(cur.val);
        return arr;
    }

    print() {
        let cur = this.head;
        while (cur.next) {
            console.log(cur.val);
            cur = cur.next;
        }
        console.log(cur.val);
        console.log(`length: ${this.length}`);
    }
}

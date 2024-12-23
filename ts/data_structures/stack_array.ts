export default class Stack<T> {
    values: Array<T | undefined>;
    index: number;
    capacity: number;

    constructor(capacity: number) {
        this.capacity = capacity;
        this.values = new Array<T>(this.capacity);
        this.index = -1;
    }

    push(val: T) {
        if (this.index >= this.capacity - 1) {
            console.log("Capacity reached!");
            return;
        }
        this.values[++this.index] = val;
    }

    pop(): T | undefined {
        if (this.index < 0 ) { 
            console.log("Stack is Empty!");
            return undefined;
        }
        let pop_val = this.values[this.index];
        this.values[this.index] = undefined;
        --this.index;
        return pop_val;
    }

    print() {
        console.log(this.values);
    }
}

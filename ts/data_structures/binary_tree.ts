class Node<T> {
    val: T;
    left: Node<T> | null;
    right: Node<T> | null;
    parent: Node<T> | null;

    constructor(val: T) {
        this.val = val;
        this.left = null;
        this.right = null;
        this.parent = null;
    }

    setLeft(node: Node<T>) {
        this.left = node;
    }

    setRight(node: Node<T>) {
        this.right = node;
    }

    setPatent(node: Node<T>) {
        this.parent = node;
    }

}

export default class UBinaryTree<T> {
    root: Node<T> | null;

    constructor() {
        this.root = null;
    }

    insert(val: T) {
        if (!this.root) {
            this.root = new Node(val);
            return;
        }

        this._insert(this.root, val);
    }

    protected _insert(node: Node<T>, val: T) {
        if (val < node.val) {
            if (!node.left) {
                node.left = new Node(val);
            } else {
                this._insert(node.left, val);
            }
        }
        else {
            if (!node.right) {
                node.right = new Node(val);
            } else {
                this._insert(node.right, val);
            }
        }
    }

    delete() {
    }

    find() {
    }

    preOrder(): T[] | null {
        if (!this.root) {
            console.log("Binary Tree is empty");
            return null;
        }
        return this._preOrder(this.root);
    }

    protected _preOrder(node: Node<T>): T[] {
        if (!node.left) {
        }

        return [];
    }



}

export default function sort(arr: number[]) {
    for (let i = 1; i < arr.length; i++) {
        for (let j = 0; j < i; j++) {
            if (arr[i] < arr[j]) {
                let temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }
        }
    }
    return arr;
}

import { assertEquals } from "https://deno.land/std@0.178.0/testing/asserts.ts";

Deno.test(function insertionSort() {
    let arr: number[] = sort([8, 7, 4, -7, 1, 9, 3, -9, 5, -3, -6, 0, -5, -8, -4, 2, -10, -2, 6, -1, 10]);
    let sorted_arr: number[] = [-10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

    assertEquals(arr, sorted_arr);
});

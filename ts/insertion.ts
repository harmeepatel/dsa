let sort = (arr: number[]) => {
    for (let i = 1; i < arr.length; i++) {
        for (let j = 0; j < i; j++) {
            if (arr[i] < arr[j]) {
                let temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }
        }
    }
    console.log(arr);
};

let arr: number[] = [
    8, 7, 4, -7, 1, 9, 3, -9, 5, -3, -6, 0, -5, -8, -4, 2, -10, -2, 6, -1, 10,
];

sort(arr);

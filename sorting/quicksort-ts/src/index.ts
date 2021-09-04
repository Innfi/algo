import assert from 'assert';


const quickSort = (input: any[], lo: number, hi: number): void => {
    if(hi <= lo) return;    
    let lt = lo;
    let gt = hi;
    const pivot = input[lo];

    let i = lo;
    while(i <= gt) {
        if(input[i] < pivot) swapElement(input, lt++, i++);
        else if (input[i] > pivot) swapElement(input, i, gt--);
        else i++;
    }

    quickSort(input, lo, lt-1);
    quickSort(input, gt+1, hi);
};

const swapElement = (input: any[], lhs: number, rhs: number): void => {
    let temp = input[lhs];
    input[lhs] = input[rhs];
    input[rhs] = temp;
};

describe('QuickSort', () => {
    it('ascending ortder', () => {
        const input: number[] = [ 4, 3, 9, 8, 1, 5, 10, 10, 3, 3, 25];
        quickSort(input, 0, input.length-1);

        assert.strictEqual(input.slice(1).every((v, i) => v >= input[i]), true);
    });
});
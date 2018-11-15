export class ReMath {
    static factorial(n) {
        return n < 2 ? 1 : n * this.factorial(n - 1);
    }
}
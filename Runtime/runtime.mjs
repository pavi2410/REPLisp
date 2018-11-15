export class ReMath {
    static factorial(n) {
        return n <= 1 ? 1 : this.factorial(n-1)
    }
}
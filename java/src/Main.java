public class Main {

    static {
        System.loadLibrary("first");
    }

    public static native int mult(int a, int b);
    public static native int sum(int a, int b);
    public static native int div(int a, int b);
    public static native int sub(int a, int b);
    public static native int sqr(int a);

    private Main() {}

    public static void main(String[] args) {
        try {
            System.out.printf("Java => dynamic Rust first.mult(4, 5) is %d\n", mult(4, 5));
            System.out.printf("Java => dynamic Rust first => dynamic Rust second.sum(4, 5) is %d\n", sum(4, 5));
            System.out.printf("Java => dynamic Rust first => dynamic JNI second.sub(10, 5) is %d\n", sub(10, 5));
            System.out.printf("Java => dynamic Rust first => runtime dynamic Rust third.div(42, 6) is %d\n", div(42, 6));
            // This one fails
            System.out.printf("Java => dynamic Rust first => runtime dynamic JNI third.sqr(16) is %d\n", sqr(16));
        } catch (Throwable e) {
            System.err.println(e.toString());
        }
    }
}

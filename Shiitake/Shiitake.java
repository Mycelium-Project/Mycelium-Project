class Shiitake {
    private static native boolean init_measurements();

    private static native double cpu_usage();
    private static native double memory_usage();
    private static native double network_usage_in();
    private static native double network_usage_out();

    static {
        System.loadLibrary("shiitake_lib");
    }
    static {
        init_measurements();
    }
}
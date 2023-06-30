
import java.util.Objects;


class Shiitake {
    private static native boolean init_measurements();

    static {
        //NEED TO HAVE DLL IN JAVA LIB PATH
        System.load(Objects.requireNonNull(Shiitake.class.getResource("libnative.so")).getFile());
    }
    static {
        init_measurements();
    }

    public static native double cpu_usage();
    public static native int cpu_frequency();
    public static native double memory_usage();
    public static native double network_usage_in();
    public static native double network_usage_out();
    public static native String os_version();
    public static native int cpu_cores();
    public static native double memory_total();

    public static void main(String[] args) {

        for (int i = 0; i < 100000; i++) {
            System.out.println("CPU Usage: " + cpu_usage());
            System.out.println("CPU Frequency: " + cpu_frequency());
            System.out.println("Memory Usage: " + memory_usage());
            System.out.println("Network Usage In: " + network_usage_in());
            System.out.println("Network Usage Out: " + network_usage_out());
            System.out.println("OS Version: " + os_version());
            System.out.println("CPU Cores: " + cpu_cores());
            System.out.println("Memory Total: " + memory_total());
            //sleep for 2 seconds
            try {
                Thread.sleep(2000);
            } catch (InterruptedException e) {
                e.printStackTrace();
                System.exit(1);
            }
        }
    }
}
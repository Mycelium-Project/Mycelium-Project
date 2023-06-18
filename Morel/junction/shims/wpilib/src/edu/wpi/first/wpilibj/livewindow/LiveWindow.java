package edu.wpi.first.wpilibj.livewindow;

import edu.wpi.first.util.sendable.Sendable;

@Deprecated
public final class LiveWindow {

    private LiveWindow() {
        throw new UnsupportedOperationException("This is a utility class!");
    }

    public static synchronized void setEnabledListener(Runnable runnable) {}

    public static synchronized void setDisabledListener(Runnable runnable) {}

    public static synchronized boolean isEnabled() {return false;}

    public static synchronized void setEnabled(boolean enabled) {}

    public static synchronized void enableTelemetry(Sendable sendable) {}

    public static synchronized void disableTelemetry(Sendable sendable) {}

    public static synchronized void disableAllTelemetry() {}

    public static synchronized void enableAllTelemetry() {}

    public static synchronized void updateValues() {}
}

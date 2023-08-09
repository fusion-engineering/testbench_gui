import matplotlib.pyplot as plt
import scipy as sp
import numpy as np
import math


def bwfilter(data, wc):
    # wc = 10
    sos = sp.signal.butter(3, wc, fs=2000, output="sos")
    # w, h = sp.signal.freqz(sos, fs=2000)
    # plt.semilogx(w, 20 * np.log10(abs(h)))
    filtered = sp.signal.sosfilt(sos, data)
    return filtered


def test_bwfilter():
    fs = 2000
    t = 2
    w1 = 3
    w2 = 200
    x = np.linspace(0, t, t * fs)
    y = np.sin(w1 * 2 * 3.14 * x) + np.sin(w2 * 2 * 3.14 * x)

    y2 = bwfilter(y, 20)
    plt.plot(x, y)
    plt.plot(x, y2)
    plt.show()
    bwfilter(data)


def process_data(df):
    df.columns = ["time", "throttle", "rpm", "current", "thrust", "torque"]
    thrust_offset = 62571
    df["thrust"] = -df["thrust"] - thrust_offset
    df["filtered"] = bwfilter(df["current"], wc=10)


def plot(df, show=True):
    plt.figure()
    ax1 = plt.subplot(2, 3, 1)
    plt.plot(df["time"], df["throttle"])
    ax1.set_title("throttle")
    ax1.set_xlabel("time [ms]")
    ax1.set_ylabel("throttle [dshot command]")

    ax2 = plt.subplot(2, 3, 2)
    plt.plot(df["time"], df["rpm"])
    ax2.set_title("speed feedback")
    ax2.set_xlabel("time [ms]")
    ax2.set_ylabel("feedback [rpm]")

    ax3 = plt.subplot(2, 3, 3)
    plt.plot(df["time"], df["current"])
    plt.plot(df["time"], df["filtered"])
    ax3.set_title("current")
    ax3.set_xlabel("time [ms]")
    ax3.set_ylabel("current [A]")

    ax4 = plt.subplot(2, 3, 4)
    plt.plot(df["time"], df["thrust"])
    ax4.set_title("thrust")
    ax4.set_xlabel("time [ms]")
    ax4.set_ylabel("thrust[N]")

    ax5 = plt.subplot(2, 3, 5)
    plt.plot(df["time"], df["torque"])
    ax5.set_title("torque")
    ax5.set_xlabel("time [ms]")
    ax5.set_ylabel("torque [Nm]")

    if show:
        plt.show()

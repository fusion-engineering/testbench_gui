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


def process_data(df):
    df.columns = ["time", "throttle", "rpm", "current", "thrust", "torque"]
    thrust_offset = 62571
    df["thrust"] = -df["thrust"] - thrust_offset
    df["filtered"] = bwfilter(df["current"], wc=10)

    # CALIBRATION DATA: thrust [grams] = 0.00964 * measurement
    df["thrust"] = df["thrust"] * 0.00964


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
    # plt.hlines(df["thrust"].mean(),0,df.shape[0],linestyles='--',colors='r')
    ax4.set_title("thrust")
    ax4.set_xlabel("time [ms]")
    ax4.set_ylabel("thrust[grams]")

    ax5 = plt.subplot(2, 3, 5)
    plt.plot(df["time"], df["torque"])
    ax5.set_title("torque")
    ax5.set_xlabel("time [ms]")
    ax5.set_ylabel("torque [Nm]")

    ax6 = plt.subplot(2, 3, 6)
    plt.plot(df["rpm"], df["thrust"])
    ax6.set_title("rpm vs thrust")
    ax6.set_xlabel("rpm")
    ax6.set_ylabel("thrust[grams]")
    if show:
        plt.show()

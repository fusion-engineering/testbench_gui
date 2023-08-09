import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import plotdata
import argparse

argParser = argparse.ArgumentParser()
argParser.add_argument("-f", "--filename", help="enter your filename here")

args = argParser.parse_args()
filename = args.filename
df = pd.read_csv(filename)
plotdata.process_data(df)
show = True
plotdata.plot(df, show)

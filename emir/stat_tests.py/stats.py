import numpy as np
import pandas as pd
import pingouin as pg
from scipy.stats import shapiro
import scikit_posthocs as sp

# https://www.reneshbedre.com/blog/friedman-test-python.html


import csv

with open("data.csv", mode="r") as file:
    csv_file = csv.reader(file)

    # Skips:
    for _ in range(2):
        next(csv_file)

    n = []
    dp_memo_avg_time = []
    dp_tab_avg_time = []
    greedy_value = []
    greedy_weight = []
    greedy_ratio = []

    greedy_value_error = []
    greedy_weight_error = []
    greedy_ratio_error = []

    for line in csv_file:
        n.append(int(line[0]))
        dp_memo_avg_time.append(float(line[7]))
        dp_tab_avg_time.append(float(line[14]))
        greedy_value.append(float(line[21]))
        greedy_weight.append(float(line[29]))
        greedy_ratio.append(float(line[37]))

        greedy_value_error.append(float(line[22].replace("%", "")))
        greedy_weight_error.append(float(line[28].replace("%", "")))
        greedy_ratio_error.append(float(line[36].replace("%", "")))

    time_dataframe = pd.DataFrame(
        {
            "n": np.array(n),
            "Memoization Data": np.array(dp_memo_avg_time),
            "Tabulation Data": np.array(dp_tab_avg_time),
            "Greedy Value": np.array(greedy_value),
            "Greedy Weight": np.array(greedy_weight),
            "Greedy Ratio": np.array(greedy_ratio),
        }
    )

    error_dataframe = pd.DataFrame(
        {
            "n": np.array(n),
            "Greedy Value": np.array(greedy_value_error),
            "Greedy Weight": np.array(greedy_weight_error),
            "Greedy Ratio": np.array(greedy_ratio_error),
        }
    )

    print("Statistical Checks on Times")

    print(f"{"Algorithm":<20} {"P-Value":<20} Conclusion")
    for column in [
        "Memoization Data",
        "Tabulation Data",
        "Greedy Value",
        "Greedy Weight",
        "Greedy Ratio",
    ]:
        p_value = shapiro(time_dataframe[column]).pvalue
        if p_value < 0.05:
            conclusion = "Not Normally Distributed"
        else:
            conclusion = "Normally Distributed"
        print(f"{column:<20} {p_value:<20.5e} {conclusion}")

    print()

    long_df = pd.melt(
        time_dataframe,
        id_vars=["n"],
        value_vars=[
            "Memoization Data",
            "Tabulation Data",
            "Greedy Value",
            "Greedy Weight",
            "Greedy Ratio",
        ],
        var_name="Algorithm",
        value_name="Time",
    )

    friedman = pg.friedman(
        data=long_df,
        dv="Time",
        within="Algorithm",
        subject="n",
    )

    print(friedman)

    print()

    posthoc_results = sp.posthoc_conover_friedman(
        a=long_df,
        y_col="Time",
        group_col="Algorithm",
        block_col="n",
        block_id_col="n",
        p_adjust="fdr_bh",
        melted=True,
    )

    print(posthoc_results)
    print()

    print("Statistical Checks on Percentage Errors")

    print(f"{"Algorithm":<20} {"P-Value":<20} Conclusion")
    for column in [
        "Greedy Value",
        "Greedy Weight",
        "Greedy Ratio",
    ]:
        p_value = shapiro(error_dataframe[column]).pvalue
        if p_value < 0.05:
            conclusion = "Not Normally Distributed"
        else:
            conclusion = "Normally Distributed"
        print(f"{column:<20} {p_value:<20.5e} {conclusion}")
    print()

    long_df = pd.melt(
        error_dataframe,
        id_vars=["n"],
        value_vars=[
            "Greedy Value",
            "Greedy Weight",
            "Greedy Ratio",
        ],
        var_name="Algorithm",
        value_name="Error",
    )

    friedman = pg.friedman(
        data=long_df,
        dv="Error",
        within="Algorithm",
        subject="n",
    )

    print(friedman)
    print()

    posthoc_results = sp.posthoc_conover_friedman(
        a=long_df,
        y_col="Error",
        group_col="Algorithm",
        block_col="n",
        block_id_col="n",
        p_adjust="fdr_bh",
        melted=True,
    )

    print(posthoc_results)

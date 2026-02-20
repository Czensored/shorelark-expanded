#!/usr/bin/env python3
import argparse
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd


def ci95(series: pd.Series) -> float:
    n = series.count()
    if n <= 1:
        return 0.0
    return 1.96 * (series.std(ddof=1) / np.sqrt(n))


def add_mean_and_ci(ax, grouped: pd.core.groupby.generic.SeriesGroupBy, label: str, color: str):
    stats = grouped.agg(["mean", ci95]).reset_index()
    x = stats["generation"].to_numpy()
    y = stats["mean"].to_numpy()
    ci = stats["ci95"].to_numpy()
    ax.plot(x, y, color=color, linewidth=2.0, label=label)
    ax.fill_between(x, y - ci, y + ci, color=color, alpha=0.2)


def add_improvement_plot(ax, df: pd.DataFrame, value_col: str, label: str, color: str):
    per_run = (
        df.sort_values(["run", "generation"])
        .groupby("run", as_index=False)
        .apply(
            lambda g: g.assign(
                improvement=g[value_col].diff()
            )
        )
        .reset_index(drop=True)
    )
    per_run = per_run.dropna(subset=["improvement"])
    grouped = per_run.groupby("generation")["improvement"]
    add_mean_and_ci(ax, grouped, label, color)


def main():
    parser = argparse.ArgumentParser(
        description="Visualize batch genetic algorithm results."
    )
    parser.add_argument("csv", type=Path, help="Input CSV from simulation-batch runner.")
    parser.add_argument(
        "--out",
        type=Path,
        default=Path("batch_trends.png"),
        help="Output PNG path (default: batch_trends.png).",
    )
    parser.add_argument(
        "--prey-count",
        type=int,
        default=40,
        help="Total prey population used in simulation (default: 40).",
    )
    args = parser.parse_args()

    df = pd.read_csv(args.csv)
    required = {
        "run",
        "generation",
        "prey_avg_fitness",
        "predator_avg_fitness",
        "prey_dead",
    }
    missing = sorted(required - set(df.columns))
    if missing:
        raise ValueError(f"Missing required columns: {', '.join(missing)}")

    df = df.sort_values(["run", "generation"]).copy()

    fig, axes = plt.subplots(2, 2, figsize=(14, 10))
    ax1, ax2, ax3, ax4 = axes.flatten()

    add_mean_and_ci(
        ax1,
        df.groupby("generation")["prey_avg_fitness"],
        label="Prey mean fitness",
        color="#1f77b4",
    )
    add_mean_and_ci(
        ax1,
        df.groupby("generation")["predator_avg_fitness"],
        label="Predator mean fitness",
        color="#d62728",
    )
    ax1.set_title("Mean Fitness vs Generation (95% CI)")
    ax1.set_xlabel("Generation")
    ax1.set_ylabel("Average fitness")
    ax1.legend()
    ax1.grid(alpha=0.25)

    add_improvement_plot(ax2, df, "prey_avg_fitness", "Prey improvement", "#1f77b4")
    add_improvement_plot(
        ax2, df, "predator_avg_fitness", "Predator improvement", "#d62728"
    )
    ax2.axhline(0.0, color="black", linewidth=1.0, alpha=0.5)
    ax2.set_title("Improvement per Generation (95% CI)")
    ax2.set_xlabel("Generation")
    ax2.set_ylabel("Delta avg fitness")
    ax2.legend()
    ax2.grid(alpha=0.25)

    final_gen = df["generation"].max()
    final_df = df[df["generation"] == final_gen]
    box_data = [
        final_df["prey_avg_fitness"].to_numpy(),
        final_df["predator_avg_fitness"].to_numpy(),
    ]
    box = ax3.boxplot(box_data, tick_labels=["Prey", "Predator"], patch_artist=True)
    box["boxes"][0].set_facecolor("#1f77b4")
    box["boxes"][0].set_alpha(0.4)
    box["boxes"][1].set_facecolor("#d62728")
    box["boxes"][1].set_alpha(0.4)
    ax3.set_title(f"Final Generation Distribution (gen={int(final_gen)})")
    ax3.set_ylabel("Average fitness")
    ax3.grid(alpha=0.25, axis="y")

    phase = (
        df.groupby("generation", as_index=False)[
            ["prey_avg_fitness", "predator_avg_fitness"]
        ]
        .mean()
    )
    sc = ax4.scatter(
        phase["prey_avg_fitness"],
        phase["predator_avg_fitness"],
        c=phase["generation"],
        cmap="viridis",
        s=28,
        alpha=0.95,
    )
    ax4.plot(
        phase["prey_avg_fitness"],
        phase["predator_avg_fitness"],
        color="#555555",
        linewidth=1.4,
        alpha=0.7,
    )
    start = phase.iloc[0]
    end = phase.iloc[-1]
    ax4.scatter(start["prey_avg_fitness"], start["predator_avg_fitness"], color="#2ca02c", s=45, label="Start")
    ax4.scatter(end["prey_avg_fitness"], end["predator_avg_fitness"], color="#ff7f0e", s=45, label="End")
    ax4.set_title("Prey-Predator Tradeoff Phase Plot")
    ax4.set_xlabel("Prey average fitness")
    ax4.set_ylabel("Predator average fitness")
    ax4.legend(loc="best")
    ax4.grid(alpha=0.25)
    cbar = fig.colorbar(sc, ax=ax4)
    cbar.set_label("Generation")

    fig.suptitle("Shorelark Batch Simulation Trends", fontsize=14)
    fig.tight_layout()
    args.out.parent.mkdir(parents=True, exist_ok=True)
    fig.savefig(args.out, dpi=180)

    run_dieout = (df.groupby("run")["prey_dead"].max() >= args.prey_count)
    dieout_runs = int(run_dieout.sum())
    total_runs = int(run_dieout.size)
    dieout_runs_pct = (100.0 * dieout_runs / total_runs) if total_runs else 0.0

    print(f"Wrote plot: {args.out}")
    print(
        "Runs with at least one full prey die-out: "
        f"{dieout_runs}/{total_runs} ({dieout_runs_pct:.2f}%)"
    )


if __name__ == "__main__":
    main()

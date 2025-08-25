import matplotlib.pyplot as plt

def main():
    names = ["libsais i32", "libsais i64", "libsais i32 OpenMP", "libsais i64 OpenMP", "divsufsort (i32)", "suffix (u32)", "bio (usize)", "psacak (u32)", "sais-drum (usize)"]
    x = list(range(9))
    running_times = [78, 86, 26, 29, 185, 364, 1092, 131, 353]
    peak_memory_usages = [8, 16, 8, 16, 8, 11.2, 21.6, 8, 16.2]
    colors = ["blue","cornflowerblue","blueviolet","violet","tomato","orange","forestgreen","olive","grey"]

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(10, 5))
    
    bars1 = ax1.bar(x, running_times, color=colors)
    ax1.set_title("Running times in seconds")
    ax1.set_xticks([]) 
    ax1.bar_label(bars1)

    bars2 = ax2.bar(x, peak_memory_usages, color=colors)
    ax2.set_title("Memory usage in gigabytes")
    ax2.set_xticks([]) 
    ax2.bar_label(bars2)

    fig.legend(ax1.patches, names, loc="upper center", ncol = 3)
    fig.tight_layout()

    # plot was manually edited to position legend outside of plots
    fig.savefig("plot.svg")


if __name__ == "__main__":
    main()

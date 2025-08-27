import matplotlib.pyplot as plt

def main():
    names = ["libsais (i32)", "libsais 8 threads (i32)", "libsais (i64)",  "libsais 8 threads (i64)", "divsufsort (i32)", "suffix (str input, u32)", "bio (usize)", "psacak (u32)", "psacak 8 threads (u32)", "sais-drum (usize)"]
    x = list(range(10))
    running_times = [78, 26, 86, 29, 185, 364, 1092, 281, 131, 353]
    peak_memory_usages = [8, 8, 16, 16, 8, 11.2, 21.6, 8, 8, 16.2]
    colors = ["blue","cornflowerblue","blueviolet","violet","tomato","orange","forestgreen","olive", "darkkhaki", "grey"]

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(10, 5))
    
    bars1 = ax1.bar(x, running_times, color=colors)
    ax1.set_title("Running time in seconds")
    ax1.set_xticks([]) 
    ax1.bar_label(bars1)
    ax1.set_ylabel("seconds")

    bars2 = ax2.bar(x, peak_memory_usages, color=colors)
    ax2.set_title("Memory usage in gigabytes")
    ax2.set_xticks([]) 
    ax2.bar_label(bars2)
    ax2.set_ylabel("gigabytes")

    fig.legend(ax1.patches, names, loc="upper center", ncol = 4)
    fig.tight_layout()

    # plot was manually edited to position legend outside of plots
    fig.savefig("plot.svg")

if __name__ == "__main__":
    main()

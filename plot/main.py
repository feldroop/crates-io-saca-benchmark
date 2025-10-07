import matplotlib.pyplot as plt
import json

# key -> (name, color)
key_to_info = {
    "None": ("none", "black"),
    "Libsais": ("libsais (i32)", "blue"),
    "Libsais64": ("libsais (i64)", "blueviolet"),
    "LibsaisOpenMp": ("libsais 8 threads (i32)", "cornflowerblue"),
    "LibsaisOpenMp64": ("libsais 8 threads (i64)", "violet"),
    "Divsufsort": ("divsufsort (i32)", "tomato"),
    "Suffix": ("suffix (u32, str input)", "orange"),
    "Bio": ("bio (usize)", "forestgreen"),
    "Psacak": ("psacak (u32)", "olive"),
    "PsacakThreads": ("psacak 8 threads (u32)", "darkkhaki"),
    "SaisDrum": ("sais_drum (u32)", "grey"),
    "SaisDrum64": ("sais_drum (u64)", "lightgrey"),
    "SufrPartial128": ("sufr 8 threads (u32, partial sort 128, written to file)", "teal"),
}

def read_library_configs_and_result_data():
    with open(f"../results.json") as f:
        file_contents = f.read()

    results = json.loads(file_contents)

    results_list = sorted(results.items(), key=lambda tup: key_to_info[tup[0]][0])

    library_configs = list(map(lambda tup: tup[0], results_list))
    results_data = list(map(lambda tup: tup[1], results_list))

    return library_configs, results_data

def main():
    library_configs, results_data = read_library_configs_and_result_data()

    running_times = list(map(lambda result: result["elapsed_time_secs"], results_data))
    peak_memory_usages = list(map(lambda result: result["peak_memory_usage_gb"], results_data))

    n = len(library_configs)
    x = list(reversed(range(n)))

    library_nice_names = list(map(lambda conf: key_to_info[conf][0], library_configs))
    library_colors = list(map(lambda conf: key_to_info[conf][1], library_configs))

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 7))
    
    bar_label_fmt = "{:.1f}"
    label_padding = 2

    bars1 = ax1.barh(x, running_times, color=library_colors)
    ax1.set_title("running time in seconds")
    ax1.set_yticks([]) 
    ax1.bar_label(bars1, fmt=bar_label_fmt, padding=label_padding)
    ax1.set_xlabel("seconds")
    ax1.margins(x=0.15)

    bars2 = ax2.barh(x, peak_memory_usages, color=library_colors)
    ax2.set_title("peak memory usage in gigabytes")
    ax2.set_yticks([]) 
    ax2.bar_label(bars2, fmt=bar_label_fmt, padding=label_padding)
    ax2.set_xlabel("gigabytes")
    ax2.margins(x=0.15)

    fig.subplots_adjust(top=0.70)
    fig.legend(
        ax1.patches, 
        library_nice_names, 
        bbox_to_anchor=(0, 0.75, 1, 0.2),
        bbox_transform=fig.transFigure, 
        loc="lower center", 
        ncol = 3
    )

    fig.tight_layout(rect=[0, 0, 1, 0.75])
    fig.savefig(f"plot.svg", bbox_inches="tight")

if __name__ == "__main__":
    main()

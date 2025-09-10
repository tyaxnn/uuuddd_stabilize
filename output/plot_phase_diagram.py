import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
from scipy.interpolate import griddata
import matplotlib.colors as mcolors
import os

# ==== 設定 ====
j_val = "0p3"
mesh_size = 200
input_file = f"./phase_diagram_data_rev/phase_diagram_{j_val}_{mesh_size}.dat"
output_dir = "./phase_diagram_plot_rev"
output_file = f"{output_dir}/phase_diagram_{j_val}_{mesh_size}.png"

# ==== 出力ディレクトリ作成 ====
os.makedirs(output_dir, exist_ok=True)

# ==== データ読み込み ====
df = pd.read_csv(input_file)

# ==== stable を数値に変換 ====
stable_categories = ["fm", "afm", "uuuddd", "ududdd", "dududd", "uudddd", "uddddd", "dudddd"]
cat_to_num = {cat: i for i, cat in enumerate(stable_categories)}
df['stable_num'] = df['stable'].map(cat_to_num)

# ==== グリッド作成 ====
n_vals = np.linspace(df['n'].min(), df['n'].max(), 200)
lambda_vals = np.linspace(df['lambda'].min(), df['lambda'].max(), 200)
grid_n, grid_lambda = np.meshgrid(n_vals, lambda_vals)

# ==== 補間 ====
points = df[['n', 'lambda']].values
values = df['stable_num'].values
grid_stable_num = griddata(points, values, (grid_n, grid_lambda), method='nearest')

# ==== カラーマップ: rainbow（カテゴリ用に離散化） ====
cmap = plt.get_cmap('rainbow', len(stable_categories))
norm = mcolors.BoundaryNorm(boundaries=np.arange(-0.5, len(stable_categories)+0.5, 1),
                            ncolors=len(stable_categories))

# ==== プロット ====
fig, ax = plt.subplots(figsize=(8, 6))
im = ax.imshow(grid_stable_num, origin='lower',
               extent=(n_vals.min(), n_vals.max(), lambda_vals.min(), lambda_vals.max()),
               cmap=cmap, norm=norm, aspect='auto')

# ==== 軸ラベルとタイトル ====
ax.set_xlabel('n', fontsize=16)
ax.set_ylabel('λ', fontsize=16)  # lambda をギリシャ文字 λ に
ax.set_title(f'Phase Diagram (j={j_val}, mesh={mesh_size})', fontsize=18)

# ==== カラーバー ====
cbar = fig.colorbar(im, ticks=np.arange(len(stable_categories)))
cbar.ax.set_yticklabels(stable_categories, fontsize=12)
cbar.set_label('Stable Phase', fontsize=14)

# ==== 軸目盛サイズ調整 ====
ax.tick_params(labelsize=12)

plt.tight_layout()

# ==== 保存 ====
plt.savefig(output_file, dpi=300)
plt.show()

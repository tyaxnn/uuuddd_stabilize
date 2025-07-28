import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
from scipy.interpolate import griddata
import matplotlib.colors as mcolors
import os

# ==== 設定 ====
lambda_val = "0p175"
mesh_size = 200
input_file = f"./lambda_phase_diagram_data/lambda_phase_diagram_{lambda_val}_{mesh_size}.dat"
output_dir = "./lambda_phase_diagram_plot"
output_file = f"{output_dir}/lambda_phase_diagram_{lambda_val}_{mesh_size}.png"

# ==== 出力ディレクトリ作成 ====
os.makedirs(output_dir, exist_ok=True)

# ==== データ読み込み ====
df = pd.read_csv(input_file)

# ==== stable を数値に変換 ====
stable_categories = ["fm", "afm", "uuuddd", "ududdd", "uudddd", "uddddd"]
cat_to_num = {cat: i for i, cat in enumerate(stable_categories)}
df['stable_num'] = df['stable'].map(cat_to_num)

# ==== グリッド作成 ====
n_vals = np.linspace(df['n'].min(), df['n'].max(), 200)
j_vals = np.linspace(df['jj'].min(), df['jj'].max(), 200)
grid_n, grid_j = np.meshgrid(n_vals, j_vals)

# ==== 補間 ====
points = df[['n', 'jj']].values
values = df['stable_num'].values
grid_stable_num = griddata(points, values, (grid_n, grid_j), method='nearest')

# ==== カラーマップ: rainbow（カテゴリ用に離散化） ====
cmap = plt.get_cmap('rainbow', len(stable_categories))
norm = mcolors.BoundaryNorm(boundaries=np.arange(-0.5, len(stable_categories)+0.5, 1),
                            ncolors=len(stable_categories))

# ==== プロット ====
fig, ax = plt.subplots(figsize=(8, 6))
im = ax.imshow(grid_stable_num, origin='lower',
               extent=(n_vals.min(), n_vals.max(), j_vals.min(), j_vals.max()),
               cmap=cmap, norm=norm, aspect='auto')

# ==== 軸ラベルとタイトル ====
ax.set_xlabel('n', fontsize=16)
ax.set_ylabel('j', fontsize=16)  # j をギリシャ文字 λ に
ax.set_title(f'Phase Diagram (lambda={lambda_val}, mesh={mesh_size})', fontsize=18)

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

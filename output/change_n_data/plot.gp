# --- 設定 ---
# データファイル名を変数として定義
datafile = "energy_vs_n_lambda0p3_j0p25_mu0.dat"

# 出力ファイル名をデータファイル名から自動生成
outputfile = datafile.".png"

# グラフのタイトルも設定
graphtitle = "Energy vs n (lambda=0.3, j=0.25, mu=0)"


# --- 描画設定 ---
# 出力形式とファイル名
set terminal pngcairo enhanced font "Arial,12" size 800,600
set output outputfile

# データファイルの区切り文字をカンマに設定
set datafile separator ","

# グラフの各種設定
set xlabel "n"
set ylabel "Energy" # Y軸のラベルをより具体的に変更
set title graphtitle
set grid
set key outside right top


# --- 描画コマンド ---
# 1列目をx軸に、2列目から11列目までをy軸としてプロット
plot datafile using 1:2 with linespoints title "fm", \
     ''       using 1:3 with linespoints title "afm", \
     ''       using 1:4 with linespoints title "uuuddd", \
     ''       using 1:5 with linespoints title "ududddd", \
     ''       using 1:6 with linespoints title "dududd", \
     ''       using 1:7 with linespoints title "uudddd", \
     ''       using 1:8 with linespoints title "uddddd", \
     ''       using 1:9 with linespoints title "dudddd"

# スクリプトの終わり
print "Graph has been generated: ", outputfile
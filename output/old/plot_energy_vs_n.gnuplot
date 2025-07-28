# === Gnuplot スクリプト ===

# データファイルと出力設定
set datafile separator ","
set title "Energy vs. n"
set xlabel "n"
set ylabel "Energy"
set grid
set key outside
set terminal pngcairo size 1000,700 enhanced font 'Arial,12'
set output "energy_vs_n_lambda0p3_j0p25_mu0.png"

# プロット
plot \
    "energy_vs_n_lambda0p3_j0p25_mu0.dat" using 1:2 with lines title "fm", \
    "" using 1:3 with lines title "afm", \
    "" using 1:4 with lines title "uuuddd", \
    "" using 1:5 with lines title "face", \
    "" using 1:6 with lines title "tri", \
    "" using 1:7 with lines title "twin", \
    "" using 1:8 with lines title "one"
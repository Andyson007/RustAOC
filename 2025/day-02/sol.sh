echo ',' | cat input - | while read -a arr -d ,; do seq $(echo $arr | sed 's/-/ /') | grep -P '^(.+)\1+$'; done | sort | uniq | paste -sd+ | bc

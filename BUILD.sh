git pull;
trunk build --release;
sudo docker run -d --name mtvtvlep -v /home/teresa/mtv-tv-leptos/dist:/usr/share/nginx/html:ro -p 9090:80 nginx:bookworm;
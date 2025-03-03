自分の環境でicedを使うとコンパイルエラーが起きる。ChatGPT先生によるとicedはWaylandというのと相性が悪いらしい。そこでX11というのを使うように強制してあげると上手く動くらしい。なので、
export WAYLAND_DISPLAY= && export DISPLAY=:0
を~/.bash_rcに入れておくとコンパイルできる。  

sudo apt install libx11-dev cmake libfreetype-dev libexpat1-dev libfontconfig1-dev mesa-vulkan-drivers  
した上で(教科書先生ありがとう)  
export WGPU_BACKEND=vulkan  
することで(ChatGPT先生ありがとう)Waylandのまま解決！  
export WGPU_BACKEND=vulkanが必要かは保留  
  
おそらくicedに限らない話として、core dumpとかの原因不明のバグはgdbとかを使うとどこでバグが発生したか特定できるらしい。とはいえ使い方はChatGPT先生に聞くしかなさそう。  

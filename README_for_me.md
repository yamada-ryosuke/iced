自分の環境でicedを使うとコンパイルエラーが起きる。ChatGPT先生によるとicedはWaylandというのと相性が悪いらしい。そこでX11というのを使うように強制してあげると上手く動くらしい。なので、
export WAYLAND_DISPLAY= && export DISPLAY=:0
を~/.bash_rcに入れておくとコンパイルできる。  

DEPENDENCIES.mdに従って依存を解決し、
export WGPU_BACKEND=vulkan  
することでWaylandのまま解決(ChatGPT先生ありがとう)  
  
おそらくicedに限らない話として、core dumpとかの原因不明のバグはgdbとかを使うとどこでバグが発生したか特定できるらしい。とはいえ使い方はChatGPT先生に聞きつつ使っていく  

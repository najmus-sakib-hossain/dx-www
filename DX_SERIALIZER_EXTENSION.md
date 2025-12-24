Good, you did really well in our dx serializer vs code extesnion - Now please do these
1. Make sure that if we put json, yml, toml or csv or other serializer in our dx serializer file then we will create llm version of our dx serilaizer, put in the file main disk and then will make human verion of that serializer from it and will put in the .dx/cache folder so that we can show the human verison infront of the user and last but not least we will also create mahcine version of the seriliazer and put in the ./dx/cache folder with file name like "dx.machine" - So, no matter any serializer put into our dx seriliazer file it will automatically translslated to dx serialiazer format
2. Make sure that when I change a thing dx serializer human version it should reflect it in the llm version and later will also generate dx serializer machine version from it and put in the dx/cache folder correctly!!!
3. And I like this dx seriliazer human version:
```toml
name                 = dxf
version              = 0.0.1
title                = Enhanced Developing Experience
description          = "Orchestratedon'tjustownyourcode"
author               = essensefromexistence
license              = MIT

[stack]              = Lang | Runtime | Compiler | Bundler | PM | Framework
javascript           = javascript/typescript | bun | tsc | vite | bun | react
python               = python | cpython | - | - | uv | django
rust                 = rust | native | rustc | - | cargo | -

[forge]
repository           = https://dx.vercel.app/essensefromexistence/dx
container            = none
pipeline                = none
tasks                = none
items                = cli | docs | examples | packages | scripts | style | tests

[style]
path                 = @/style
engine               = atomic | enhanced | logical
themes               = dx | vercel | claude

[ui]
path                 = @/components/ui
components           = button | card | modal | navbar | footer

[media]
images_path          = @/public/images/*
images               = dummy1.jpg | dummy2.png
videos_path          = @/public/videos/*
videos               = dummy1.mp4
sounds_path          = @/public/sounds/*
sounds               = dummy1.wav
assets_path          = @/public/assets/*
assets               = dummy1.asset

[i18n]
locales_path         = @/locales
locales_default      = en-US
locales_dev          = en-US
locales_prod         = all
ttses_path           = ./ttses
ttses_default        = en-US
ttses_dev            = en-US | bn-BD
ttses_prod           = all

[icon]
path                 = @/components/icons
pack                 = lucide-react
variant              = default

[font]
path                 = @/font
default              = Inter
primary              = Manrope
secondary            = RobotoMono

workspace            = frontend/www | frontend/mobile
editors              = vscode | vim | gitpod | github-codespace | replit | firebase-studio | cursor | windsurf | stackblitz
```

Instead of current human version:
```toml
[config]
name        = dx
version     = 0.0.1
title       = "Enhanced Developing Experience"
description = "Orchestrate don't just own your code"
author      = essensefromexistence
workspace   = frontend/www, frontend/mobile
editors     = vscode, vim, gitpod, github-codespace, replit, firebase-studio, cursor, windsurf, stackblitz

[forge]
name   repository                                     container  ci_cd  tasks  items                                               
forge  https://dx.vercel.app/essensefromexistence/dx  none       none   none   cli, docs, examples, packages, scripts, style, tests

[stack]
name   path     engine  themes  
stack  @/stack  atomic  enhanced

[style]
name   path     engine  themes  
style  @/style  atomic  enhanced

[ui]
name  path             components                         
ui    @/components/ui  button, card, modal, navbar, footer

[media]
name   images_path        images                  videos_path        videos      sounds_path        sounds      assets_path        assets      
media  @/public/images/*  dummy1.jpg, dummy2.png  @/public/videos/*  dummy1.mp4  @/public/sounds/*  dummy1.wav  @/public/assets/*  dummy1.asset

[i18n]
name  locales_path  locales_default  locales_dev  locales_prod  ttses_path  ttses_default  ttses_dev     ttses_prod
i18n  @/locales     en-US            en-US        all           ./ttses     en-US          en-US, bn-BD  all       

[icon]
name  path                pack          variant
icon  @/components/icons  lucide-react  default

[font]
name  path    default  primary  secondary 
font  @/font  Inter    Manrope  RobotoMono
```

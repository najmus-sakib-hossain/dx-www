serializer vscode extension+professional folder structure + weakness
font
dx-tools with cli
style(serializer)

www

cli+forge+serializer+py

I approved that dx serilaizer is the best serializer human version - so please do playground test of dx serilaizer llm version vs with toon and dx serilaizer machine version vs with rkyv and test if dx serializer is really faster than them - If so then our dx seriliazer is the best serializer in the world!!!

dx-py-runtime
dx-py-package-manager
dx-py-workspace
dx-py-test-runner
dx-py-compability

dx-js-runtime
dx-js-package-manager
dx-js-workspace
dx-js-test-runner
dx-js-compability


Dx LPS and Cli
1. Cli
2. Forge
3. Dx Tools
4. Cli
5. Forge


So, currently at crates dx folder we have dx cli code and crates forge folder we forge rust code so please make sure that dx cli gives controls to all dx tools and make the forge rust crate to run as a daemon that watche for files changes as VCS, and control other dx tools, use dummy tools instaces as I still need to do some work before actually integratting those tools to forge and then make dx forge can do all things that it suppose to do like traffic branch and everything else and in the cli we are not adding chat for now but put a logic so that when we create dx cli ai chat it can be updated with ai chat correctly without any hassle and connect forge with out dx extension at root extension folder and rename it to dx from dx serializer as we are adding other tools previously it was just working on serializer but now it will do much more so please name it to dx extension and make sure that cli, forge, serializer and extension works correctly!!!

Good, now we will control both "dx" config file and .dx folder and that .dx folder we will put all of our cahcing folder:
1. .dx/www
2. .dx/extension
3. .dx/cli

4. .dx/cache
5. .dx/runtime
6. .dx/package-manager
7. .dx/workspace
8. .dx/test-runner
9. .dx/compability

10. .dx/serializer
11. .dx/forge
12. .dx/style
13. .dx/ui
14. .dx/font
15. .dx/media
16. .dx/icon
17. .dx/i18n
18. .dx/auth
19. .dx/test
20. .dx/driven
21. .dx/generator

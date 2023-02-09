//cargo，cargo.io
    //通过release profile来自定义构建。
    //在http://cargo.io/上发布库
    //通过workspaces组织大工程。
    //从http://cargo.io/来安装库。
    //使用自定义命令扩展cargo。



//通过release profile来自定义构建。
    //release profile
        //是预定义的。
        //可自定义：可使用不同的配置，对代码编译拥有更多的控制。
    //每个profile的配置都独立于其它的profile。

    //Cargo主要的两个profile：
        //dev profile：适用于开发：cargo build
        //release profile：适用于发布：cargo build —release
    
    //自定义profile
        //针对每个profile，Cargo都提供了默认的配置。
        //如果想自定义xxx profile的配置：
            //可以在Cargo.toml里添加[profile.xxx]区块，在里面覆盖默认配置的子集。
        //例子（以下在toml文件中的[dependencies]下写
            /* 
            [dependencies]

            [profile.dev]   
                        //opt-level决定了rust参数对代码执行那种层次的优化，从0~3，优化的层度越高，就需要更多的编译时间。
                            //开发阶段经常编译代码，为缩短编译时间，所以通常优化层度低。
                            //发布产品时，最好是花费更多的时间来编译程序，因为发布是编译一次，但以后会多次运行编译后的程序。
            opt-level = 1



            [profile.release]
            opt-level = 3
            */

        //对于每个配置的默认值和完整选项，请参见：http://doc.rust-lang.org/cargo/














//在http://cargo.io/上发布库
    //cartes.io
        //可以通过发布包共享你的代码。
        //crate的注册表在http://crates.io/
            //他会分发已注册的包的源代码。
            //主要托管开源的代码。



    //发布crate到crate.io
        //创建并设置Crate.io账号
            //发布crate前，需要在crate.io创建账号并获得API token。
            //运行命令：cargo login [你的API token]
                //通知cargo，你的API token存储在本地 ~/.cargo/credentials
            //API token可以在http://crates.io/ 进行撤销。（一旦泄露及撤销， 我的API token=cioUNWmBBFThCY5ojIh1NRcdViFPnY77ECQ)

        //为新的crate添加元数据    
            //在发布crate之前，需要在Crago.toml的[package]区域为crate添加一些元数据：
                //crate需要唯一的名称：name （需要先在toml文件中的name命一个名，因为需要在http://crates.io/ 上独一无二的，当然原名称是独一无二的也行，是否为独一无二可以猜http://crates.io/ 上查找是否重名。）。
                //description：一两句话即可，会出现在crate搜索的结果里。
                //license：需要提供许可证标识符（可到http://spdx.org/licenses 查找。
                    //可指定多个license：用OR
                //version
                //author
            //发布：crago publish 命令。
            
        //发布到Crates.io
            //crate一旦发布，就是永久性的：该版本无法覆盖，代码无法删除。
                //目的：依赖于该版本的项目可继续正常工作。
//命令提示符：发布到Crates.io实操（展开）。
/* 
PS D:\rust\projects\study\cargo> cargo login cioUNWmBBFThCY5ojIh1NRcdViFPnY77ECQ
       Login token for `crates.io` saved
PS D:\rust\projects\study\cargo> cargo publish
    Updating `ustc` index
warning: manifest has no description, license, license-file, documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
error: 3 files in the working directory contain changes that were not yet committed into git:

Cargo.lock
Cargo.toml
src\main.rs

to proceed despite this and include the uncommitted changes, pass the `--allow-dirty` flag
PS D:\rust\projects\study\cargo> cargo publish --allow-dirty
    Updating `ustc` index
warning: manifest has no description, license, license-file, documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
   Packaging ljf v0.1.0 (D:\rust\projects\study\cargo)
   Verifying ljf v0.1.0 (D:\rust\projects\study\cargo)
   Compiling ljf v0.1.0 (D:\rust\projects\study\cargo\target\package\ljf-0.1.0)
    Finished dev [optimized + debuginfo] target(s) in 0.68s
   Uploading ljf v0.1.0 (D:\rust\projects\study\cargo)
error: failed to publish to registry at https://crates.io

Caused by:
  the remote server responded with an error: missing or empty metadata fields: description, license. Please see https://doc.rust-lang.org/cargo/reference/manifest.html for how to upload metadata
PS D:\rust\projects\study\cargo> cargo publish --allow-dirty
    Updating `ustc` index
warning: manifest has no documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
   Packaging ljf v0.1.0 (D:\rust\projects\study\cargo)
   Verifying ljf v0.1.0 (D:\rust\projects\study\cargo)
   Compiling ljf v0.1.0 (D:\rust\projects\study\cargo\target\package\ljf-0.1.0)
    Finished dev [optimized + debuginfo] target(s) in 0.95s
   Uploading ljf v0.1.0 (D:\rust\projects\study\cargo)
error: failed to publish to registry at https://crates.io

Caused by:
  the remote server responded with an error: A verified email address is required to publish crates to crates.io. Visit https://crates.io/me to set and verify your email address.PS D:\rust\projects\study\cargo> cargo publish --allow-dirty
    Updating `ustc` index
warning: manifest has no documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
   Packaging ljf v0.1.0 (D:\rust\projects\study\cargo)
   Verifying ljf v0.1.0 (D:\rust\projects\study\cargo)
   Compiling ljf v0.1.0 (D:\rust\projects\study\cargo\target\package\ljf-0.1.0)
    Finished dev [optimized + debuginfo] target(s) in 0.63s
   Uploading ljf v0.1.0 (D:\rust\projects\study\cargo)
error: failed to publish to registry at https://crates.io

Caused by:
  the remote server responded with an error: A verified email address is required to publish crates to crates.io. Visit https://crates.io/me to set and verify your email address.
PS D:\rust\projects\study\cargo> cargo publish --allow-dirty
    Updating `ustc` index
warning: manifest has no documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
   Packaging ljf v0.1.0 (D:\rust\projects\study\cargo)
   Verifying ljf v0.1.0 (D:\rust\projects\study\cargo)
   Compiling ljf v0.1.0 (D:\rust\projects\study\cargo\target\package\ljf-0.1.0)
    Finished dev [optimized + debuginfo] target(s) in 0.53s
   Uploading ljf v0.1.0 (D:\rust\projects\study\cargo)
PS D:\rust\projects\study\cargo>
*/

        //发布已存在crate的新版本
            //修改crate后，需要先修改Cargo.toml里面的version值，再进行重新发布。
            //参照http://semver.org/ 来使用你的语义版本。
            //在执行cargo publish进行发布。

        //使用cargo yank从Crates.io撤回版本
            //不可以删除crate之前的版本。
            //但可以防止其它项目把它作为新的依赖：yank（撤回）一个crate版本
                //防止新项目依赖于该版本。
                //已经存在项目可继续将其作为依赖。（并可下载）
            //yank意味着：
                //所有已经产生Cargo.lock的项目都不会中断。
                //任何将来生成的Cargo.lock文件都不会使用被yank的版本。
            //命令：
                //yank 一个版本（不会删除任何代码）：cargo yank --vers 0.1.0
                //取消yank：cargo yank --vers 0.1.0 --undo
//命令提示符：yank于取消yank实操（展开）
/* 
PS D:\rust\projects\study\cargo> cargo yank --vers 0.1.0
    Updating `ustc` index
        Yank ljf@0.1.0
PS D:\rust\projects\study\cargo> cargo yank --vers 0.1.0 --undo
    Updating `ustc` index
      Unyank ljf@0.1.0
*/




//通过workspaces组织大工程
    //Cargo工作空间（workspaces）
        //cargo工作空间：帮助管理多个相互关联且需要协同开发的crate。
        //cargo工作空间是一套共享同一个Crago.lock和输出文件夹的包。

    //创建工作空间
        //有多种方式来组建工作空间例如：1个二进制crate，2个库crate
            //二进制crate：main函数，依赖于其它2个库crate。
            //其中1个库crate提供add_one函数。
            //另外1个库crate提供add_tow函数。

    //在工作空间中依赖外部crate
        //工作空间只有一个Cargo.lock文件，再工作空间的顶层目录。
            //保证工作空间内所有crate使用的依赖的版本都相同。
            //工作空间内所有的crate相互兼容。

    //为工作空间添加测试













//从http://cargo.io/ 来安装库。
    //从CRATES.IO安装二进制crate。
        //命令：cargo install
        //来源：http://cargo.io/
        //限制：只能安装具有二进制目标（binary target）的crate。
        //二进制目标binary target：是一个可运行程序
            //由拥有src/main.rs或其它被指定为二进制文件的crate生成。
        //通常：README里有关于crate的描述：
            //拥有library traget。
            //拥有binary target。
            //两者兼备。

    //cargo install
        //cargo install安装的二进制文件存放在根目录的bin文件夹。
        //如果你用rustup安装的Rust，没有任何自定义配置，那么二进制存放目录是$HOME/.cargo/bin
            //要确保该目录再环境变量$PATH中。





//使用自定义命令扩展cargo。
    //cargo被设计成可以使用子命令来扩展。
    //例如：如果$PATH中的某个二进制式cargo-something，你可以像子命令一样运行：
        //-cargo something
    //类似这样的自定义命令可以通过该命令列出：cargo --list
    //有点：可以使用cargo install来安装扩展，像内置工具一样来运行。





















fn main() {
    println!("Hello, world!");
}

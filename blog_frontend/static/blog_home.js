let posts = document.getElementsByClassName("blog_post");
let blog_overlays = document.getElementsByClassName("overlay");
for(let i = 0; i<posts.length;i++){
    //this makes the gradient appear over blogposts if the blog-post is bigger then max-height.
    if (get_em_unit_size(posts[i],posts[i].clientHeight) < get_em_unit_size(posts[i],get_style(posts[i],"max-height"))){
        blog_overlays[i].classList.toggle('display_inactive');
    }
}

//need to remake this bad. 
let blog_nav = document.getElementById("blog_nav");
let more_content = document.getElementById("more_content");
document.getElementById("arrow")
    .addEventListener("click",function(){
        blog_nav.classList.toggle("display_active");
        blog_nav.classList.toggle("display_inactive");
        more_content.classList.toggle("more_content_inactive")
});

function get_em_unit_size(element, measured_component){
    let font_size = parseInt(get_style(element,"font-size"));
    return parseInt(measured_component)/font_size;
}

function get_style(element,style) {
    return window.getComputedStyle(element, null).getPropertyValue(style);
}

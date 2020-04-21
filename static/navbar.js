let desktop_nav = document.getElementById("desktop_nav");
document.getElementById("hamburger_icon_container")
    .addEventListener("click",function(){
       desktop_nav.classList.toggle("display_active");
       desktop_nav.classList.toggle("display_inactive");
    });

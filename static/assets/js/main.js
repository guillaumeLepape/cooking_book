/*=============== SHOW MENU ===============*/
const navMenu = document.getElementById('nav-menu'),
      navToggle = document.getElementById('nav-toggle'),
      navClose = document.getElementById('nav-close')

/* Menu show */
navToggle.addEventListener('click', () =>{
   navMenu.classList.add('show-menu')
})

/* Menu hidden */
navClose.addEventListener('click', () =>{
   navMenu.classList.remove('show-menu')
})

/*=============== SEARCH ===============*/
const search = document.getElementById('search'),
      searchBtn = document.getElementById('search-btn'),
      searchClose = document.getElementById('search-close')

/* Search show */
searchBtn.addEventListener('click', () =>{
   search.classList.add('show-search')
})

/* Search hidden */
searchClose.addEventListener('click', () =>{
   search.classList.remove('show-search')
})

/*=============== LOGIN ===============*/
const login = document.getElementById('login'),
      loginBtn = document.getElementById('login-btn'),
      loginClose = document.getElementById('login-close')

/* Login show */
loginBtn.addEventListener('click', () =>{
   login.classList.add('show-login')
})

/* Login hidden */
loginClose.addEventListener('click', () =>{
   login.classList.remove('show-login')
})

function searchInRecipes() {
   let input = document.getElementById("clear-input");

   let allRecipes = document.getElementById("all-recipes");

   for (let recipe of allRecipes.getElementsByClassName("recipe")) {
      let summary = recipe.getElementsByClassName("recipe__summary")[0];

      let txtValue = summary.textContent || summary.innerText;

      if (txtValue.toUpperCase().indexOf(input.value.toUpperCase()) > -1) {
         recipe.style.display = "";
      } else {
         recipe.style.display = "none";
      }
   }

   const handleInputChange = (e) => {
      if (e.target.value && !input.classList.contains("clear-input--touched")) {
         input.classList.add("clear-input--touched")
      }
      else if (!e.target.value && input.classList.contains("clear-input--touched")) {
         input.classList.remove("clear-input--touched")
      } 
   }

   input.addEventListener("input", handleInputChange)
}

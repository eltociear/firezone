"use strict";(self.webpackChunknew_docs=self.webpackChunknew_docs||[]).push([[9199],{3905:(e,t,n)=>{n.d(t,{Zo:()=>u,kt:()=>f});var r=n(7294);function i(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function a(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){i(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,r,i=function(e,t){if(null==e)return{};var n,r,i={},o=Object.keys(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||(i[n]=e[n]);return i}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(i[n]=e[n])}return i}var s=r.createContext({}),c=function(e){var t=r.useContext(s),n=t;return e&&(n="function"==typeof e?e(t):a(a({},t),e)),n},u=function(e){var t=c(e.components);return r.createElement(s.Provider,{value:t},e.children)},p={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},m=r.forwardRef((function(e,t){var n=e.components,i=e.mdxType,o=e.originalType,s=e.parentName,u=l(e,["components","mdxType","originalType","parentName"]),m=c(n),f=i,d=m["".concat(s,".").concat(f)]||m[f]||p[f]||o;return n?r.createElement(d,a(a({ref:t},u),{},{components:n})):r.createElement(d,a({ref:t},u))}));function f(e,t){var n=arguments,i=t&&t.mdxType;if("string"==typeof e||i){var o=n.length,a=new Array(o);a[0]=m;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l.mdxType="string"==typeof e?e:i,a[1]=l;for(var c=2;c<o;c++)a[c]=n[c];return r.createElement.apply(null,a)}return r.createElement.apply(null,n)}m.displayName="MDXCreateElement"},4913:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>s,contentTitle:()=>a,default:()=>p,frontMatter:()=>o,metadata:()=>l,toc:()=>c});var r=n(7462),i=(n(7294),n(3905));const o={title:"Uninstall",sidebar_position:4},a=void 0,l={unversionedId:"administer/uninstall",id:"administer/uninstall",title:"Uninstall",description:"To completely remove Firezone and its configuration files, run the [uninstall.sh",source:"@site/docs/administer/uninstall.md",sourceDirName:"administer",slug:"/administer/uninstall",permalink:"/administer/uninstall",draft:!1,editUrl:"https://github.com/firezone/firezone/tree/master/docs/administer/uninstall.md",tags:[],version:"current",sidebarPosition:4,frontMatter:{title:"Uninstall",sidebar_position:4},sidebar:"tutorialSidebar",previous:{title:"Upgrade",permalink:"/administer/upgrade"},next:{title:"Troubleshoot",permalink:"/administer/troubleshoot"}},s={},c=[],u={toc:c};function p(e){let{components:t,...n}=e;return(0,i.kt)("wrapper",(0,r.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("p",null,"To completely remove Firezone and its configuration files, run the ",(0,i.kt)("a",{parentName:"p",href:"https://github.com/firezone/firezone/blob/master/scripts/uninstall.sh"},"uninstall.sh\nscript"),":"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-bash"},'sudo /bin/bash -c "$(curl -fsSL https://github.com/firezone/firezone/raw/master/scripts/uninstall.sh)"\n')),(0,i.kt)("p",null,(0,i.kt)("strong",{parentName:"p"},"Warning"),": This will irreversibly destroy ALL Firezone data and can't be\nundone."))}p.isMDXComponent=!0}}]);
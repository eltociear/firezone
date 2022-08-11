"use strict";(self.webpackChunknew_docs=self.webpackChunknew_docs||[]).push([[6794],{3905:(e,t,r)=>{r.d(t,{Zo:()=>s,kt:()=>f});var n=r(7294);function o(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function a(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function i(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?a(Object(r),!0).forEach((function(t){o(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):a(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function p(e,t){if(null==e)return{};var r,n,o=function(e,t){if(null==e)return{};var r,n,o={},a=Object.keys(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||(o[r]=e[r]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(o[r]=e[r])}return o}var l=n.createContext({}),c=function(e){var t=n.useContext(l),r=t;return e&&(r="function"==typeof e?e(t):i(i({},t),e)),r},s=function(e){var t=c(e.components);return n.createElement(l.Provider,{value:t},e.children)},u={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},d=n.forwardRef((function(e,t){var r=e.components,o=e.mdxType,a=e.originalType,l=e.parentName,s=p(e,["components","mdxType","originalType","parentName"]),d=c(r),f=o,y=d["".concat(l,".").concat(f)]||d[f]||u[f]||a;return r?n.createElement(y,i(i({ref:t},s),{},{components:r})):n.createElement(y,i({ref:t},s))}));function f(e,t){var r=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=r.length,i=new Array(a);i[0]=d;var p={};for(var l in t)hasOwnProperty.call(t,l)&&(p[l]=t[l]);p.originalType=e,p.mdxType="string"==typeof e?e:o,i[1]=p;for(var c=2;c<a;c++)i[c]=r[c];return n.createElement.apply(null,i)}return n.createElement.apply(null,r)}d.displayName="MDXCreateElement"},1544:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>l,contentTitle:()=>i,default:()=>u,frontMatter:()=>a,metadata:()=>p,toc:()=>c});var n=r(7462),o=(r(7294),r(3905));const a={title:"HAProxy",sidebar_position:3},i=void 0,p={unversionedId:"deploy/reverse-proxies/haproxy",id:"deploy/reverse-proxies/haproxy",title:"HAProxy",description:"The following is an example configuration for the",source:"@site/docs/deploy/reverse-proxies/haproxy.md",sourceDirName:"deploy/reverse-proxies",slug:"/deploy/reverse-proxies/haproxy",permalink:"/deploy/reverse-proxies/haproxy",draft:!1,editUrl:"https://github.com/firezone/firezone/tree/master/docs/deploy/reverse-proxies/haproxy.md",tags:[],version:"current",sidebarPosition:3,frontMatter:{title:"HAProxy",sidebar_position:3},sidebar:"tutorialSidebar",previous:{title:"Traefik",permalink:"/deploy/reverse-proxies/traefik"},next:{title:"Authenticate",permalink:"/authenticate/"}},l={},c=[],s={toc:c};function u(e){let{components:t,...r}=e;return(0,o.kt)("wrapper",(0,n.Z)({},s,r,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("p",null,"The following is an example configuration for the\n",(0,o.kt)("a",{parentName:"p",href:"https://www.haproxy.org/"},"HAProxy")," proxy. we assume\n",(0,o.kt)("inlineCode",{parentName:"p"},"default['firezone']['phoenix']['port']")," to be ",(0,o.kt)("inlineCode",{parentName:"p"},"13000")," and the proxy running on\nthe same host as the Firezone app."),(0,o.kt)("p",null,"There is not SSL termination in this configuration so a previous proxy will\nneed to terminate the SSL connection."),(0,o.kt)("p",null,"You can also configure HAProxy to handle the SSL termination as explained\n",(0,o.kt)("a",{parentName:"p",href:"https://www.haproxy.com/blog/haproxy-ssl-termination/"},"here")," but take into\naccount that the ",(0,o.kt)("inlineCode",{parentName:"p"},"pem")," file expected by ",(0,o.kt)("inlineCode",{parentName:"p"},"ssl crt")," option needs to contain\nboth the ",(0,o.kt)("inlineCode",{parentName:"p"},"crt")," and ",(0,o.kt)("inlineCode",{parentName:"p"},"key")," file."),(0,o.kt)("p",null,(0,o.kt)("inlineCode",{parentName:"p"},"/etc/haproxy/haproxy.cfg"),":"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-conf"},"defaults\n    mode http\n\nfrontend app1\n    bind *:80\n    option forwardfor\n    default_backend             backend_app1\n\nbackend backend_app1\n    server mybackendserver 127.0.0.1:13000\n")))}u.isMDXComponent=!0}}]);
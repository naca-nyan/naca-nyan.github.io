import{u as l}from"./composables.1fb3d189.js";import{a}from"./entry.38337790.js";const S=e=>Object.fromEntries(Object.entries(e).filter(([,t])=>t!==void 0)),s=(e,t)=>(n,r)=>(l(()=>e({...S(n),...r.attrs},r)),()=>{var o,i;return t?(i=(o=r.slots).default)==null?void 0:i.call(o):null}),g={accesskey:String,autocapitalize:String,autofocus:{type:Boolean,default:void 0},class:String,contenteditable:{type:Boolean,default:void 0},contextmenu:String,dir:String,draggable:{type:Boolean,default:void 0},enterkeyhint:String,exportparts:String,hidden:{type:Boolean,default:void 0},id:String,inputmode:String,is:String,itemid:String,itemprop:String,itemref:String,itemscope:String,itemtype:String,lang:String,nonce:String,part:String,slot:String,spellcheck:{type:Boolean,default:void 0},style:String,tabindex:String,title:String,translate:String};a({name:"NoScript",inheritAttrs:!1,props:{...g,title:String,body:Boolean,renderPriority:[String,Number]},setup:s((e,{slots:t})=>{var o;const n={...e},r=(((o=t.default)==null?void 0:o.call(t))||[]).filter(({children:i})=>i).map(({children:i})=>i).join("");return r&&(n.children=r),{noscript:[n]}})});a({name:"Link",inheritAttrs:!1,props:{...g,as:String,crossorigin:String,disabled:Boolean,fetchpriority:String,href:String,hreflang:String,imagesizes:String,imagesrcset:String,integrity:String,media:String,prefetch:{type:Boolean,default:void 0},referrerpolicy:String,rel:String,sizes:String,title:String,type:String,methods:String,target:String,body:Boolean,renderPriority:[String,Number]},setup:s(e=>({link:[e]}))});a({name:"Base",inheritAttrs:!1,props:{...g,href:String,target:String},setup:s(e=>({base:e}))});const m=a({name:"Title",inheritAttrs:!1,setup:s((e,{slots:t})=>{var r,o,i;return{title:((i=(o=(r=t.default)==null?void 0:r.call(t))==null?void 0:o[0])==null?void 0:i.children)||null}})});a({name:"Meta",inheritAttrs:!1,props:{...g,charset:String,content:String,httpEquiv:String,name:String,body:Boolean,renderPriority:[String,Number]},setup:s(e=>{const t={...e};return t.httpEquiv&&(t["http-equiv"]=t.httpEquiv,delete t.httpEquiv),{meta:[t]}})});a({name:"Style",inheritAttrs:!1,props:{...g,type:String,media:String,nonce:String,title:String,scoped:{type:Boolean,default:void 0},body:Boolean,renderPriority:[String,Number]},setup:s((e,{slots:t})=>{var o,i,p;const n={...e},r=(p=(i=(o=t.default)==null?void 0:o.call(t))==null?void 0:i[0])==null?void 0:p.children;return r&&(n.children=r),{style:[n]}})});a({name:"Head",inheritAttrs:!1,setup:(e,t)=>()=>{var n,r;return(r=(n=t.slots).default)==null?void 0:r.call(n)}});a({name:"Html",inheritAttrs:!1,props:{...g,manifest:String,version:String,xmlns:String,renderPriority:[String,Number]},setup:s(e=>({htmlAttrs:e}),!0)});a({name:"Body",inheritAttrs:!1,props:{...g,renderPriority:[String,Number]},setup:s(e=>({bodyAttrs:e}),!0)});const f=""+new URL("icon.d4cad487.png",import.meta.url).href;export{m as T,f as _};

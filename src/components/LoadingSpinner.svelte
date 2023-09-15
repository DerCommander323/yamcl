<div class="lds-dual-ring"></div>

<style>
.lds-dual-ring {
  display: inline-block;
  width: 16px;
  height: 16px;
}
.lds-dual-ring:after {
  content: " ";
  display: block;
  width: 18px;
  height: 18px;
  margin: 2px;
  border-radius: 50%;
  border: 2px solid #fff;
  border-color: #fff transparent #fff transparent;
  animation: lds-dual-ring 1.0s linear infinite;
}
@keyframes lds-dual-ring {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
</style>
# https://qiita.com/P_Fermat/items/ff4e32051ff3565488ba

using ParameterizedFunctions, DifferentialEquations
using Plots

# 常微分方程式の定義
g = @ode_def Lorenz begin
    dx = p*(y-x)
    dy = x*(r-z) - y
    dz = x*y - b*z
  end p r b

# 問題定義
u0 = [0.0;1.01;0.0]
tspan = (0.0,100.0)
p = [10.0,25.0,7/3]
prob = ODEProblem(g,u0,tspan,p)

# 計算
sol = solve(prob,Tsit5(),reltol=1e-8,abstol=1e-8)

function plt()
    plot(sol,vars=(1,2,3), title="Lorenz Attractor", 
        xlabel="x", ylabel="y", zlabel="z", label="")
    savefig("lorenz.png")
end

plt()

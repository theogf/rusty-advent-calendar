

function parseinput(input)
    pos = []
    instructions = []
    for line in input
        isempty(line) && continue
        m = match(r"fold along ([xy])=(\d+)", line)
        if isnothing(m)
            push!(pos, parse.(Int, split(line, ",")))
            # ret[x+1, y+1] = true
        else
            push!(instructions, first(m[1]) => parse(Int, m[2]))
        end
    end
    corner = [655 * 2 + 1, 447 * 2 + 1]
    paper = falses(corner...)
    for p in pos
        paper[(p .+ 1)...] = true
    end
    paper, corner, instructions
end

function fold!(paper, corner, along, value)
    tpaper = along == 'y' ? paper' : paper
    along == 'x' ? (corner[1] -= value + 1) : (corner[2] -= value + 1)
    tpaper[1:value, :] .|= tpaper[2*value+1:-1:value+2, :]
    tpaper[2*value+1:-1:value+2, :] .= false
end

function crop_paper(paper, corner)
    reverse(paper[1:corner[1], 1:corner[2]], dims=2)
end

function print_paper(paper, corner)
    display(crop_paper(paper, corner)')
end

input = readlines(joinpath(@__DIR__, "../data/input.data"))
paper, corner, folds = parseinput(input)
corner |> display

for fold in folds
    fold!(paper, corner, fold...)
    # print_paper(paper, corner)q
end
# print_paper(paper, corner)

using GLMakie
using Colors
# fig, ax, plt = heatmap(crop_paper(paper, corner))
fig = Figure()
ax = Axis3(fig[1,1], xgridvisible=false, aspect=:data)
input = readlines(joinpath(@__DIR__, "../data/input.data"))
paper, corner, folds = parseinput(input)
hidedecorations!(ax)
hidespines!(ax)
# zlims!(ax, -100, 100)
# xlims!(ax, -corner[1]/ 2, corner[1] / 2)
# ylims!(ax, -corner[2]/ 2, corner[2] / 2)
# hidespines!(ax)
nz = 5
z = range(0, 0.1, length=nz)
# plt = volume!(mat)
nsteps = 100
dt = 0.002
fig |> display
record(fig, joinpath(@__DIR__, "viz.gif"); framerate=50) do io
for (i, fold) in enumerate(folds)
    axis, val = fold
    if axis == 'x'
        plt1 = heatmap!(ax, -val:0, 1:corner[2], crop_paper(paper, corner), colormap=cgrad(:deep), transparency=true)
        plt2 = heatmap!(ax, 0:val, 1:corner[2], crop_paper(paper, corner), colormap=cgrad(:deep, alpha=0.5), transparency=true)
        display(fig)
        half_axis = Vec3([0, -1, 0])
        half_line = Vec3([-val, 0, 0])
        for i in 1:nsteps
            m_ϵ = qrotation(half_axis, i * π / nsteps)
            rotate!(plt2, m_ϵ)
            # sleep(dt)
            recordframe!(io)
        end
        plt1[:visible] = false
        plt2[:visible] = false
    else 
        plt1 = heatmap!(ax, 1:corner[2], -val:0, crop_paper(paper, corner), colormap=cgrad(:deep), transparency=true)
        plt2 = heatmap!(ax, 1:corner[2], 0:val, colormap=cgrad(:deep, alpha=0.5), crop_paper(paper, corner), transparency=true)
        display(fig)
        half_axis = Vec3([1, 0, 0])
        for i in 1:nsteps
            m_ϵ = qrotation(half_axis, i * π / nsteps)
            rotate!(plt2, m_ϵ)
            # sleep(dt)
            recordframe!(io)
        end
        plt1[:visible] = false
        plt2[:visible] = false
    end
    fold!(paper, corner, fold...)
end
heatmap!(ax, -corner[1]/2:corner[1]/2, -corner[2]/2:corner[2]/2, crop_paper(paper, corner), colormap=cgrad(:deep))
for i in 1:10
    recordframe!(io)
end
end
fig

-- Robot script for Ursus
-- Provides robot.tick(ctx) that returns JSON

local M = {}

function M.tick(ctx)
    return {
        status = "ok",
        ctx = ctx,
        robot = "ursus_praefectus",
        sys = {
            temperature = ctx.temperature or 0,
            cpu_load = ctx.cpu_load or 0,
        }
    }
end

return M
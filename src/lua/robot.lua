-- Robot script for Ursus
-- Provides robot.tick(ctx) that returns JSON

local M = {}

function M.tick(ctx)
    return {
        status = "ok",
        ctx = ctx,
        robot = "ursus_praefectus"
    }
end

return M